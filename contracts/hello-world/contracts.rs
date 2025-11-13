use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol, Vec,
};

#[contracttype]
pub enum DataKey {
    Poll(Symbol), 
    Voter(Symbol, Address),
}

#[contracttype]
#[derive(Clone)]
pub struct Poll {
    pub creator: Address,
    pub title: soroban_sdk::String,
    pub options: Vec<soroban_sdk::String>, 
    pub votes: Map<soroban_sdk::String, u32>, 
    pub is_active: bool,
}

// --- HỢP ĐỒNG CHÍNH ---

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn create_poll(
        env: Env,
        creator: Address,
        poll_id: Symbol,
        title: soroban_sdk::String,
        options: Vec<soroban_sdk::String>,
    ) {
        creator.require_auth();

        let storage = env.storage().persistent();
        let key = DataKey::Poll(poll_id.clone());

        // Đảm bảo poll_id chưa tồn tại
        if storage.has(&key) {
            panic!("Poll already exists");
        }

        // Khởi tạo Map để lưu trữ kết quả bầu chọn
        let mut votes: Map<soroban_sdk::String, u32> = Map::new(&env);
        for option in options.iter() {
            votes.set(option, 0); // Khởi tạo số phiếu của mỗi lựa chọn là 0
        }

        let new_poll = Poll {
            creator,
            title,
            options,
            votes,
            is_active: true,
        };

        // Lưu poll vào storage
        storage.set(&key, &new_poll);
    }

    pub fn vote(env: Env, voter: Address, poll_id: Symbol, choice: soroban_sdk::String) {
        voter.require_auth();

        let storage = env.storage().persistent();
        let poll_key = DataKey::Poll(poll_id.clone());
        let voter_key = DataKey::Voter(poll_id.clone(), voter.clone());

        // Kiểm tra xem poll có tồn tại không
        let mut poll: Poll = storage.get(&poll_key).unwrap_or_else(|| {
            panic!("Poll not found");
        });

        // Kiểm tra xem poll có đang hoạt động không
        if !poll.is_active {
            panic!("Voting is closed");
        }

        // 🚨 Tính năng bảo mật: Kiểm tra xem người dùng đã bỏ phiếu chưa
        if storage.has(&voter_key) {
            panic!("Voter already voted in this poll");
        }

        // Kiểm tra xem lựa chọn có hợp lệ không
        if !poll.votes.contains_key(&choice) {
            panic!("Invalid voting choice");
        }

        // Tăng số phiếu lên 1
        let current_votes = poll.votes.get(&choice).unwrap().unwrap();
        poll.votes.set(choice, current_votes + 1);

        // Đánh dấu người dùng đã bỏ phiếu
        storage.set(&voter_key, &true);

        // Cập nhật Poll
        storage.set(&poll_key, &poll);
    }


    pub fn get_result(env: Env, poll_id: Symbol) -> Map<soroban_sdk::String, u32> {
        let storage = env.storage().persistent();
        let poll_key = DataKey::Poll(poll_id.clone());

        // Lấy poll từ storage
        let poll: Poll = storage.get(&poll_key).unwrap_or_else(|| {
            panic!("Poll not found");
        });

        // Trả về map kết quả
        poll.votes
    }

    /// (Tùy chọn) Tính năng đóng bầu chọn
    pub fn close_poll(env: Env, creator: Address, poll_id: Symbol) {
        creator.require_auth();

        let storage = env.storage().persistent();
        let poll_key = DataKey::Poll(poll_id.clone());
        
        let mut poll: Poll = storage.get(&poll_key).unwrap_or_else(|| {
            panic!("Poll not found");
        });

        // Chỉ người tạo mới được đóng poll
        if poll.creator != creator {
            panic!("Only creator can close poll");
        }

        poll.is_active = false;
        storage.set(&poll_key, &poll);
    }
}