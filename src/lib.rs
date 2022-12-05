#![no_std]
use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec, IntoVal, RawVal, Bytes};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn check(env: Env) -> Vec<RawVal> {
        let ledger = env.ledger();
        let timestamp = ledger.timestamp();
        let key = "target".into_val(&env);
        let startkey = "start".into_val(&env);

        if let Some(target) = env.data().get::<_, u64>(key) {
            if let Some(start) = env.data().get::<_, u64>(startkey) {
                let targetval = target.unwrap() as i64;
                let remaining = targetval - timestamp as i64;
                vec![&env, symbol!("Start").into_val(&env), start.into_val(&env), symbol!("Current").into_val(&env), timestamp.into_val(&env), 
                symbol!("Target").into_val(&env), targetval.into_val(&env), symbol!("Remaining").into_val(&env),
                remaining.into_val(&env)]
            } else {
                vec![&env, symbol!("InitError").into_val(&env)]
            }
        } else {
            vec![&env, symbol!("InitError").into_val(&env)]
        }

    }

    pub fn init(env: Env, seconds: u64, data: Bytes) -> Symbol {
        let ledger = env.ledger();
        let timestamp = ledger.timestamp();
        if env.data().has("target".into_val(&env)) {
            return symbol!("AlreadySet")
        }
        env.data().set("start".into_val(&env), timestamp);
        env.data().set("target".into_val(&env), timestamp+seconds);
        env.data().set("data".into_val(&env), data);
        symbol!("Success")
    }

    pub fn get(env: Env) -> Vec<RawVal> {
        let ledger = env.ledger();
        let timestamp = ledger.timestamp();
        let key = "target".into_val(&env);

        if let Some(target) = env.data().get::<_, u64>(key) {
            let targetval = target.unwrap() as i64;
            let remaining = targetval - timestamp as i64;
            if remaining <= 0 {
                let data_key = "data".into_val(&env);
                if let Some(data) = env.data().get::<_, Bytes>(data_key) {
                    return vec![&env, symbol!("Success").into_val(&env), data.into_val(&env)];
                }           
                return vec![&env, symbol!("Error").into_val(&env), symbol!("DataError").into_val(&env)];
            }                 
            return vec![&env, symbol!("Error").into_val(&env), symbol!("NotReady").into_val(&env)];
        }
        return vec![&env, symbol!("Error").into_val(&env), symbol!("InitError").into_val(&env)];
    }
}