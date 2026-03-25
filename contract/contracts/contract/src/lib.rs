#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env};

// ID của native asset (XLM) trên Stellar
// Testnet / Futurenet: CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
// Mainnet: CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA
const NATIVE_ASSET_ID: &str = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC";

#[contract]
pub struct FundContract;

#[contractimpl]
impl FundContract {
    /// Hàm nộp tiền vào quỹ (native XLM)
    /// amount = 5000 (stroops) = 0.0005 XLM
    pub fn fund(env: Env, from: Address, amount: i128) {
        // Yêu cầu xác thực chữ ký từ người nộp tiền
        from.require_auth();

        // Kiểm tra số tiền phải đúng 5000 stroops
        if amount != 5000 {
            panic!("Amount must be exactly 5000 stroops (0.0005 XLM)");
        }

        // Lấy địa chỉ của contract (quỹ sẽ nhận tiền)
        let contract_address = env.current_contract_address();

        // Tạo client cho native asset (XLM)
        let native_asset = token::Client::new(&env, &Address::from_str(&env, NATIVE_ASSET_ID));

        // Chuyển tiền từ 'from' sang contract
        native_asset.transfer(&from, &contract_address, &amount);

        // (Tùy chọn) Bạn có thể log event nếu cần
        // env.events().publish((symbol_short!("funded"),), (from, amount));
    }

    /// (Tùy chọn) Hàm xem số dư XLM của quỹ
    pub fn balance(env: Env) -> i128 {
        let contract_address = env.current_contract_address();
        let native_asset = token::Client::new(&env, &Address::from_str(&env, NATIVE_ASSET_ID));
        native_asset.balance(&contract_address)
    }
}