#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, Map};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Comment {
    pub author: Address,
    pub content: String,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Idea {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub category: String,
    pub author: Address,
    pub votes: Map<Address, i32>, // Menggunakan Map: 1 untuk Upvote, -1 untuk Downvote
    pub comments: Vec<Comment>,
    pub is_voting_closed: bool,   // Status untuk mengunci voting
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Idea(u64),    
    IdeaList,     
    IdeaCounter,  
}

#[contract]
pub struct IdeaBoardContract;

#[contractimpl]
impl IdeaBoardContract {
    
    // 1. Read: Mengambil semua ide
    pub fn get_ideas(env: Env) -> Vec<Idea> {
        let idea_list: Vec<u64> = env.storage().instance().get(&DataKey::IdeaList).unwrap_or(Vec::new(&env));
        let mut all_ideas = Vec::new(&env);
        
        for i in 0..idea_list.len() {
            let id = idea_list.get(i).unwrap();
            if let Some(idea) = env.storage().instance().get::<_, Idea>(&DataKey::Idea(id)) {
                all_ideas.push_back(idea);
            }
        }
        
        all_ideas
    }

    // 2. Create: Membuat ide baru
    pub fn create_idea(env: Env, author: Address, title: String, description: String, category: String) -> String {
        author.require_auth();

        if title.len() == 0 || description.len() == 0 || category.len() == 0 {
            return String::from_str(&env, "Error: Title, description, and category cannot be empty");
        }

        let mut id_counter: u64 = env.storage().instance().get(&DataKey::IdeaCounter).unwrap_or(1);
        let current_id = id_counter;

        id_counter += 1;
        env.storage().instance().set(&DataKey::IdeaCounter, &id_counter);

        let idea = Idea {
            id: current_id,
            title,
            description,
            category,
            author,
            votes: Map::new(&env), // Inisialisasi Map kosong
            comments: Vec::new(&env),
            is_voting_closed: false, // Voting terbuka secara default
        };
        
        env.storage().instance().set(&DataKey::Idea(current_id), &idea);
        
        let mut idea_list: Vec<u64> = env.storage().instance().get(&DataKey::IdeaList).unwrap_or(Vec::new(&env));
        idea_list.push_back(current_id);
        env.storage().instance().set(&DataKey::IdeaList, &idea_list);
        
        String::from_str(&env, "Idea successfully submitted!")
    }

    // 3. Update: Edit ide (Hanya Author)
    pub fn edit_idea(env: Env, caller: Address, id: u64, new_title: String, new_desc: String, new_category: String) -> String {
        caller.require_auth();

        if new_title.len() == 0 || new_desc.len() == 0 || new_category.len() == 0 {
            return String::from_str(&env, "Error: Fields cannot be empty");
        }

        let key = DataKey::Idea(id);

        if let Some(mut idea) = env.storage().instance().get::<_, Idea>(&key) {
            if idea.author != caller {
                return String::from_str(&env, "Error: Unauthorized. You are not the author");
            }

            idea.title = new_title;
            idea.description = new_desc;
            idea.category = new_category;

            env.storage().instance().set(&key, &idea);
            return String::from_str(&env, "Idea updated successfully");
        }

        String::from_str(&env, "Error: Idea not found")
    }

    // 4. Fitur Upvote, Downvote, dan Hapus Vote
    // vote_value: 1 (Upvote), -1 (Downvote), 0 (Hapus Vote)
    pub fn cast_vote(env: Env, caller: Address, id: u64, vote_value: i32) -> String {
        caller.require_auth();

        if vote_value != 1 && vote_value != -1 && vote_value != 0 {
            return String::from_str(&env, "Error: Invalid vote value. Use 1 (up), -1 (down), or 0 (remove)");
        }

        let key = DataKey::Idea(id);
        
        if let Some(mut idea) = env.storage().instance().get::<_, Idea>(&key) {
            if idea.is_voting_closed {
                return String::from_str(&env, "Error: Voting is closed for this idea");
            }

            let mut votes = idea.votes;
            
            if vote_value == 0 {
                votes.remove(caller.clone()); // Mencabut vote
            } else {
                votes.set(caller.clone(), vote_value); // Menimpa vote lama dengan yang baru
            }
            
            idea.votes = votes;
            env.storage().instance().set(&key, &idea);
            
            return String::from_str(&env, "Vote successfully recorded");
        }
        
        String::from_str(&env, "Error: Idea not found")
    }

    // 5. Fitur Author: Menutup Voting
    pub fn close_voting(env: Env, caller: Address, id: u64) -> String {
        caller.require_auth();
        let key = DataKey::Idea(id);

        if let Some(mut idea) = env.storage().instance().get::<_, Idea>(&key) {
            if idea.author != caller {
                return String::from_str(&env, "Error: Unauthorized. You are not the author");
            }

            idea.is_voting_closed = true;
            env.storage().instance().set(&key, &idea);
            return String::from_str(&env, "Voting closed successfully");
        }

        String::from_str(&env, "Error: Idea not found")
    }

    // 6. Komentar
    pub fn add_comment(env: Env, caller: Address, id: u64, content: String) -> String {
        caller.require_auth();

        if content.len() == 0 {
            return String::from_str(&env, "Error: Comment content cannot be empty");
        }

        let key = DataKey::Idea(id);

        if let Some(mut idea) = env.storage().instance().get::<_, Idea>(&key) {
            let new_comment = Comment {
                author: caller.clone(),
                content,
            };
            
            let mut comments = idea.comments;
            comments.push_back(new_comment);
            idea.comments = comments;

            env.storage().instance().set(&key, &idea);
            return String::from_str(&env, "Comment added successfully");
        }
        
        String::from_str(&env, "Error: Idea not found")
    }

    // 7. Delete
    pub fn delete_idea(env: Env, caller: Address, id: u64) -> String {
        caller.require_auth();
        let key = DataKey::Idea(id);

        if let Some(idea) = env.storage().instance().get::<_, Idea>(&key) {
            if idea.author == caller {
                env.storage().instance().remove(&key);
                
                let mut idea_list: Vec<u64> = env.storage().instance().get(&DataKey::IdeaList).unwrap_or(Vec::new(&env));
                for i in 0..idea_list.len() {
                    if idea_list.get(i).unwrap() == id {
                        idea_list.remove(i);
                        env.storage().instance().set(&DataKey::IdeaList, &idea_list);
                        break;
                    }
                }
                
                return String::from_str(&env, "Idea deleted successfully");
            } else {
                return String::from_str(&env, "Error: Unauthorized. You are not the author");
            }
        }
        
        String::from_str(&env, "Error: Idea not found")
    }
}

mod test;