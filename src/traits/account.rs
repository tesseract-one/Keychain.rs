pub trait Account {
  fn supports_addresses(&self) -> bool;
}

pub trait AccountWithAddresses: Account {
  type Address;
  type AddressId;

  fn new_address(&self, id: Self::AddressId) -> Self::Address;
}