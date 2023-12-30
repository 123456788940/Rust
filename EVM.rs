use sha2::{Digest, Sha256};

struct EVM {
    contract_accounts: Vec<ContractAccount>,
    EOAs: Vec<EOA>,
    state: Vec<String>,
}

struct ContractAccount {
    code: usize,
}

struct EOA {
    pub_key: String,
    priv_key: String,
}

impl EVM {
    pub fn new() -> Self {
        EVM {
        contract_accounts: Vec::new(),
        EOAs: Vec::new(),
        state: Vec::new(),

    }
}


   pub fn add_transaction(&mut self, receiver: &EOA, sender: &EOA, amount: usize) {
    let transactions = format!("{} sent {} from/to {}", sender.priv_key, amount, receiver.pub_key);
      self.state.push(transactions);
   }
}

impl EOA {
    pub fn new() -> Self {
        let pub_key = EOA::compute_hash("seed");
        let priv_key = EOA::compute_hash("privKey");
        EOA {priv_key, pub_key}

    }

    fn compute_hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

fn main() {
    let mut evm = EVM::new();

    let sender = EOA::new();
    let receiver = EOA::new();

    evm.add_transaction(&sender, &receiver, 0);
 
  

    // Print the state after the transaction
    for transactions in &evm.state {
        println!("{}", transactions);
       
    }
    
}
