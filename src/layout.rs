use borsh::{BorshSerialize, BorshDeserialize};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct AccountFlags: u64 {
        const INITIALIZED     = 1 << 0;
        const MARKET         = 1 << 1;
        const OPEN_ORDERS    = 1 << 2;
        const REQUEST_QUEUE  = 1 << 3;
        const EVENT_QUEUE    = 1 << 4;
        const BIDS           = 1 << 5;
        const ASKS           = 1 << 6;
    }
}

// Manually implement BorshSerialize and BorshDeserialize for AccountFlags
impl BorshSerialize for AccountFlags {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.bits().serialize(writer)
    }
}

impl BorshDeserialize for AccountFlags {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let bits = u64::deserialize(buf)?;
        Ok(AccountFlags::from_bits_truncate(bits))
    }

    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let bits = u64::deserialize_reader(reader)?;
        Ok(AccountFlags::from_bits_truncate(bits))
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MarketStateV3 {
    pub padding_1: [u8; 5],
    pub account_flags: AccountFlags,
    pub own_address: [u8; 32],
    pub vault_signer_nonce: i64,
    pub base_mint: [u8; 32],
    pub quote_mint: [u8; 32],
    pub base_vault: [u8; 32],
    pub base_deposits_total: i64,
    pub base_fees_accrued: i64,
    pub quote_vault: [u8; 32],
    pub quote_deposits_total: i64,
    pub quote_fees_accrued: i64,
    pub quote_dust_threshold: i64,
    pub request_queue: [u8; 32],
    pub event_queue: [u8; 32],
    pub bids: [u8; 32],
    pub asks: [u8; 32],
    pub base_lot_size: i64,
    pub quote_lot_size: i64,
    pub fee_rate_bps: i64,
    pub referrer_rebate_accrued: i64,
    pub padding_2: [u8; 7],
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct OpenOrders {
    pub padding_1: [u8; 5],
    pub account_flags: AccountFlags,
    pub market: [u8; 32],
    pub owner: [u8; 32],
    pub base_token_free: i64,
    pub base_token_total: i64,
    pub quote_token_free: i64,
    pub quote_token_total: i64,
    pub free_slot_bits: [u8; 16],
    pub is_bid_bits: [u8; 16],
    pub orders: [[u8; 16]; 128],
    pub client_ids: [i64; 128],
    pub referrer_rebate_accrued: i64,
    pub padding_2: [u8; 7],
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SwapLayout {
    pub instruction: u8,
    pub amount_in: i64,
    pub min_amount_out: i64,
}

pub type PublicKey = [u8; 32];

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountLayout {
    pub mint: PublicKey,
    pub owner: PublicKey,
    pub amount: i64,
    pub delegate_option: i32,
    pub delegate: PublicKey,
    pub state: u8,
    pub is_native_option: i32,
    pub is_native: i64,
    pub delegated_amount: i64,
    pub close_authority_option: i32,
    pub close_authority: PublicKey,
}

// Helper functions for working with these layouts
impl MarketStateV3 {
    pub fn is_initialized(&self) -> bool {
        self.account_flags.contains(AccountFlags::INITIALIZED)
    }

    pub fn is_market(&self) -> bool {
        self.account_flags.contains(AccountFlags::MARKET)
    }
}

impl OpenOrders {
    pub fn is_initialized(&self) -> bool {
        self.account_flags.contains(AccountFlags::INITIALIZED)
    }

    pub fn is_open_orders(&self) -> bool {
        self.account_flags.contains(AccountFlags::OPEN_ORDERS)
    }
}

// Constants that might be useful
pub const ACCOUNT_FLAGS_LAYOUT_SIZE: usize = 8;
pub const MARKET_STATE_V3_LAYOUT_SIZE: usize = 388;
pub const OPEN_ORDERS_LAYOUT_SIZE: usize = 3228;
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LiquidityStateV4 {
    status: i64,
    nonce: i64,
    order_num: i64,
    depth: i64,
    coin_decimals: i64,
    pc_decimals: i64,
    state: i64,
    reset_flag: i64,
    min_size: i64,
    vol_max_cut_ratio: i64,
    amount_wave_ratio: i64,
    coin_lot_size: i64,
    pc_lot_size: i64,
    min_price_multiplier: i64,
    max_price_multiplier: i64,
    system_decimals_value: i64,
    min_separate_numerator: i64,
    min_separate_denominator: i64,
    trade_fee_numerator: i64,
    trade_fee_denominator: i64,
    pnl_numerator: i64,
    pnl_denominator: i64,
    swap_fee_numerator: i64,
    swap_fee_denominator: i64,
    need_take_pnl_coin: i64,
    need_take_pnl_pc: i64,
    total_pnl_pc: i64,
    total_pnl_coin: i64,
    pool_open_time: i64,
    punish_pc_amount: i64,
    punish_coin_amount: i64,
    orderbook_to_init_time: i64,
    swap_coin_in_amount: u128,
    swap_pc_out_amount: u128,
    swap_coin2_pc_fee: i64,
    swap_pc_in_amount: u128,
    swap_coin_out_amount: u128,
    swap_pc2_coin_fee: i64,
    pool_coin_token_account: [u8; 32],
    pool_pc_token_account: [u8; 32],
    coin_mint_address: [u8; 32],
    pc_mint_address: [u8; 32],
    lp_mint_address: [u8; 32],
    amm_open_orders: [u8; 32],
    serum_market: [u8; 32],
    serum_program_id: [u8; 32],
    amm_target_orders: [u8; 32],
    pool_withdraw_queue: [u8; 32],
    pool_temp_lp_token_account: [u8; 32],
    amm_owner: [u8; 32],
    pnl_owner: [u8; 32],
}

impl LiquidityStateV4 {
    pub fn serum_market(&self) -> &[u8; 32] {
        &self.serum_market
    }

    pub fn pool_coin_token_account(&self) -> &[u8; 32] {
        &self.pool_coin_token_account
    }

    pub fn pool_pc_token_account(&self) -> &[u8; 32] {
        &self.pool_pc_token_account
    }

    pub fn amm_open_orders(&self) -> &[u8; 32] {
        &self.amm_open_orders
    }

    pub fn amm_target_orders(&self) -> &[u8; 32] {
        &self.amm_target_orders
    }

    pub fn pool_withdraw_queue(&self) -> &[u8; 32] {
        &self.pool_withdraw_queue
    }

    pub fn pool_temp_lp_token_account(&self) -> &[u8; 32] {
        &self.pool_temp_lp_token_account
    }

    pub fn amm_owner(&self) -> &[u8; 32] {
        &self.amm_owner
    }

    pub fn pnl_owner(&self) -> &[u8; 32] {
        &self.pnl_owner
    }

    pub fn coin_mint_address(&self) -> &[u8; 32] {
        &self.coin_mint_address
    }

    pub fn pc_mint_address(&self) -> &[u8; 32] {
        &self.pc_mint_address
    }

    pub fn coin_decimals(&self) -> i64 {
        self.coin_decimals
    }

    pub fn pc_decimals(&self) -> i64 {
        self.pc_decimals
    }
}

