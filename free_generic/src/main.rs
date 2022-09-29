// For "real world" examples, see this project of mine: https://github.com/MonarchDevelopment/SquireCore/blob/main/squire_lib/src/identifiers.rs

use std::marker::PhantomData;

// extern crate uuid;
use uuid::Uuid;

/// A generic type-checked wrapper around a Uuid (to reduce boilerplate and code duplication)
pub struct TypeId<T>(pub Uuid, PhantomData<T>);

/// A type-checked Uuid for Customers
pub type CustomerId = TypeId<Customer>;
/// A type-checked Uuid for rounds
pub type OrderId = TypeId<Order>;

pub struct Customer {
    id: CustomerId,
    orders: Vec<OrderId>,
    /* Likely more fields */
}

pub struct Order {
    id: OrderId,
    /* Likely more fields */
}

fn main() {
    let id = Uuid::new_v4();

    // Convert a Uuid to a typed id
    let mut customer = Customer {
        id: id.into(),
        orders: Vec::new(),
    };
    let order = Order { id: id.into() };

    customer.orders.push(order.id);

    // We can cast typed ids back to Uuid's if needed
    assert_eq!(id, *customer.id);
    assert_eq!(id, *order.id);

    // Back we *can't* directly compare
    // assert_eq!(id, customer.id);
    // assert_eq!(id, order.id);
    // assert_eq!(order.id, customer.id);

    // Nor can we mistake a Uuid or a different typed id for an OrderId
    // assert!(customer.has_order(id));

    // Instead, we must have an OrderId or explicitly convert an id to an OrderId
    assert!(customer.has_order(order.id));
    assert!(customer.has_order(id.into()));
}

impl Customer {
    pub fn has_order(&self, o_id: OrderId) -> bool {
        self.orders.iter().find(|&o| *o == o_id).is_some()
    }
}

// Various helpful methods and traits that we want our typed ids to share
// Most of these methods mirror that of Uuid's
// Note that this is a bit overkill for just two types, but this scales extremely well

impl<T> TypeId<T> {
    /// Creates a new typed id from a Uuid
    pub fn new(id: Uuid) -> Self {
        Self(id, PhantomData)
    }
}

impl<T> Clone for TypeId<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T> Copy for TypeId<T> {}

use std::hash::Hash;
impl<T> Hash for TypeId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T> PartialEq for TypeId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for TypeId<T> {}

use std::ops::Deref;
impl<T> Deref for TypeId<T> {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<TypeId<T>> for Uuid {
    fn from(other: TypeId<T>) -> Uuid {
        other.0
    }
}

impl<T> From<Uuid> for TypeId<T> {
    fn from(other: Uuid) -> TypeId<T> {
        TypeId(other, PhantomData)
    }
}
