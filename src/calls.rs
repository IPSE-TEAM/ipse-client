use codec::Encode;
use core::marker::PhantomData;
use frame_support::Parameter;
use sp_runtime::traits::{AtLeast32Bit, Scale};
use sp_runtime::{
    generic::Header,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic,
};
use sub_runtime::ipse::Miner as SubMiner;
use sub_runtime::ipse::Order;
use substrate_subxt::{balances::AccountData, DefaultExtra, Runtime};
use substrate_subxt::{
    balances::{Balances, BalancesEventsDecoder},
    module,
    system::{System, SystemEventsDecoder},
    Call, Store,
};

pub type AccountId = <IpseRuntime as System>::AccountId;
pub type Balance = <IpseRuntime as Balances>::Balance;

#[derive(Encode, Store)]
pub struct MinersStore<T: Ipse> {
    #[store(returns = Option<SubMiner<Balance>>)]
    pub key: AccountId,
    pub _runtime: PhantomData<T>,
}

#[derive(Encode, Store)]
pub struct OrdersStore<T: Ipse> {
    #[store(returns = Vec<Order<AccountId, Balance>>)]
    pub _runtime: PhantomData<T>,
}

#[derive(Encode, Call)]
pub struct CreateOrderCall<T: Ipse> {
    pub _runtime: PhantomData<T>,
    pub key: Vec<u8>,
    pub merkle_root: [u8; 32],
    pub data_length: u64,
    pub miners: Vec<AccountId>,
    pub days: u64,
}

#[derive(Encode, Call)]
pub struct DeleteCall<T: Ipse> {
    pub _runtime: PhantomData<T>,
    pub order_id: u64,
}

#[module]
pub trait Ipse: System + Balances {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IpseRuntime;

pub trait Timestamp: System {
    type Moment: Parameter
        + Default
        + AtLeast32Bit
        + Scale<Self::BlockNumber, Output = Self::Moment>
        + Copy;
}

impl Ipse for IpseRuntime {}

impl Runtime for IpseRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;
}

impl System for IpseRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = pallet_indices::address::Address<Self::AccountId, u32>;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Balances for IpseRuntime {
    type Balance = u128;
}

impl Timestamp for IpseRuntime {
    type Moment = u128;
}
