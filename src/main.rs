#![allow(dead_code, unused_variables, unused_mut)]

use std::fmt::Debug;

/// Entry point for Chapter 10.2: Traits: Defining Shared Behavior
///
/// A trait describes behavior that several types share. This file covers
/// defining and implementing traits, default implementations, traits as
/// parameters and return values, trait bounds and conditional implementations.
fn main() {
    defining_and_implementing_a_trait();
    default_implementations();
    traits_as_parameters();
    trait_bound_syntax();
    multiple_bounds_and_where_clauses();
    returning_types_that_implement_traits();
    orphan_rule();
}

/// # Defining a Trait
/// - A trait groups method signatures that define a set of behavior.
/// - Each signature ends with a semicolon instead of a body. Every type that
///   implements the trait must provide its own body.
/// - A trait can have several methods, one per line.
/// - Traits are similar to interfaces in other languages, with some
///   differences.
trait Describe {
    fn describe(&self) -> String;
}

struct Sword {
    name: String,
    damage: u32,
}

struct Potion {
    name: String,
    healing: u32,
}

/// # Implementing a Trait on a Type
/// - The syntax is `impl TraitName for TypeName`, followed by a block with a
///   body for each method of the trait.
/// - Each type gets its own behavior behind the same method name.
impl Describe for Sword {
    fn describe(&self) -> String {
        format!("{} (damage {})", self.name, self.damage)
    }
}

impl Describe for Potion {
    fn describe(&self) -> String {
        format!("{} (heals {})", self.name, self.healing)
    }
}

fn defining_and_implementing_a_trait() {
    println!("\n{:=>80}", "");
    println!("defining_and_implementing_a_trait()\n");

    let sword = Sword {
        name: String::from("Iron sword"),
        damage: 12,
    };
    let potion = Potion {
        name: String::from("Red potion"),
        healing: 30,
    };

    // The trait must be in scope to call its methods.
    // In this example everything lives in the same module, so the trait is
    // already in scope, but if the trait would be in some other module we
    // would need to import it. When you import the struct `Sword` the traits
    // it implements are not implicitly imported.
    println!("{}", sword.describe());
    println!("{}", potion.describe());
}

/// A trait with a default implementation.
///
/// # Default Implementations
/// - A method can have a body in the trait. Types then use it unless they
///   override it.
/// - To use the default, write an empty `impl Trait for Type {}` block.
/// - Overriding uses the same syntax as implementing a method without a
///   default. The default cannot be called from the overriding method.
/// - A default method can call other methods of the same trait, even ones
///   without a default. Here `summary` builds on the required `name`.
trait Creature {
    fn name(&self) -> String;

    fn summary(&self) -> String {
        format!("(A creature called {})", self.name())
    }
}

struct Goblin;

struct Dragon {
    age: u32,
}

impl Creature for Goblin {
    fn name(&self) -> String {
        String::from("Goblin")
    }
    // `summary` uses the default implementation.
}

impl Creature for Dragon {
    fn name(&self) -> String {
        String::from("Dragon")
    }

    fn summary(&self) -> String {
        format!("(A {}-year-old {} breathing fire)", self.age, self.name())
    }
}

fn default_implementations() {
    println!("\n{:=>80}", "");
    println!("default_implementations()\n");

    let goblin = Goblin;
    let dragon = Dragon { age: 300 };
    println!("{}", goblin.summary());
    println!("{}", dragon.summary());
}

/// # Traits as Parameters
/// - `impl Trait` as a parameter type accepts any type that implements the
///   trait.
/// - The body can only use what the trait offers.
/// - Calling with a type that does not implement the trait is a compile-time
///   error.
fn show_item(item: &impl Describe) {
    println!("Item: {}", item.describe());
}

fn traits_as_parameters() {
    println!("\n{:=>80}", "");
    println!("traits_as_parameters()\n");

    let sword = Sword {
        name: String::from("Steel sword"),
        damage: 20,
    };
    let potion = Potion {
        name: String::from("Blue potion"),
        healing: 15,
    };
    show_item(&sword);
    show_item(&potion);

    // `Goblin` does not implement `Describe`. Uncomment to see the error:
    // show_item(&Goblin);
}

/// # Trait Bound Syntax
/// - `impl Trait` is shorthand for a trait bound: `fn f<T: Trait>(item: &T)`.
/// - The longer form is needed when parameters must share one type.
///   With `impl Trait`, `a` and `b` below could be different types. With
///   `T: Describe`, both must be the same type.
fn compare_items(a: &impl Describe, b: &impl Describe) {
    println!("{} vs {}", a.describe(), b.describe());
}

fn compare_same_kind<T: Describe>(a: &T, b: &T) {
    println!("Same kind: {} vs {}", a.describe(), b.describe());
}

fn trait_bound_syntax() {
    println!("\n{:=>80}", "");
    println!("trait_bound_syntax()\n");

    let sword = Sword {
        name: String::from("Iron sword"),
        damage: 12,
    };
    // Today an axe is just another kind of Sword.
    let axe = Sword {
        name: String::from("Battle axe"),
        damage: 18,
    };
    let potion = Potion {
        name: String::from("Red potion"),
        healing: 30,
    };

    compare_items(&sword, &potion); // Different types are fine.
    compare_same_kind(&sword, &axe);

    // `T` must be one type, so this is an error. Uncomment to see it:
    // compare_same_kind(&sword, &potion);
}

/// # Multiple Bounds and `where` Clauses
/// - Several traits are required with `+`: `T: Describe + Debug`.
/// - Many bounds make the signature hard to read. A `where` clause moves them
///   after the return type.
/// - `impl Trait` works with `+` too: `&(impl Describe + Debug)`.
#[derive(Debug)]
struct Shield {
    name: String,
    defense: u32,
}

impl Describe for Shield {
    fn describe(&self) -> String {
        format!("{} (defense {})", self.name, self.defense)
    }
}

fn inspect_item<T: Describe + Debug>(item: &T) {
    println!("Describe: {}", item.describe());
    println!("Debug:    {item:?}");
}

fn trade<T, U>(offered: &T, wanted: &U) -> String
where
    T: Describe + Debug,
    U: Describe,
{
    format!("Trade {} for {}", offered.describe(), wanted.describe())
}

fn multiple_bounds_and_where_clauses() {
    println!("\n{:=>80}", "");
    println!("multiple_bounds_and_where_clauses()\n");

    let shield = Shield {
        name: String::from("Oak shield"),
        defense: 8,
    };
    let potion = Potion {
        name: String::from("Red potion"),
        healing: 30,
    };

    inspect_item(&shield);
    println!("{}", trade(&shield, &potion));

    // `Potion` does not derive `Debug`, so it cannot be inspected.
    // inspect_item(&potion); // Uncomment to see the error.
}

/// # Returning Types That Implement Traits
/// - `-> impl Trait` returns some type that implements the trait, without
///   naming it. The caller only knows about the trait.
/// - The function must return one single concrete type. Returning a `Sword`
///   in one branch and a `Potion` in another does not compile.
fn make_starter_item() -> impl Describe {
    Sword {
        name: String::from("Wooden sword"),
        damage: 3,
    }
}

// Uncomment to see the error:
//
//   error[E0308]: `if` and `else` have incompatible types
//
// fn make_item(weapon: bool) -> impl Describe {
//     if weapon {
//         Sword { name: String::from("Dagger"), damage: 5 }
//     } else {
//         Potion { name: String::from("Water"), healing: 1 }
//     }
// }

fn returning_types_that_implement_traits() {
    println!("\n{:=>80}", "");
    println!("returning_types_that_implement_traits()\n");

    let item = make_starter_item();
    println!("Starter item: {}", item.describe());

    // The concrete type is hidden, so its fields are not accessible.
    // println!("{}", item.damage); // Uncomment to see the error.
}

/// # The Orphan Rule
/// - A trait can be implemented on a type only if the trait or the type is
///   local to our crate.
/// - `impl Describe for Vec<u32>` is fine: `Describe` is ours.
/// - `impl Display for Vec<u32>` is not: both come from the standard library.
/// - This coherence rule stops other crates from breaking our code by
///   providing conflicting implementations.
impl Describe for Vec<u32> {
    fn describe(&self) -> String {
        format!("A bag with {} items", self.len())
    }
}

// Uncomment to see the error:
//
//   error[E0117]: only traits defined in the current crate can be implemented
//   for types defined outside of the crate
//
// impl Display for Vec<u32> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "bag")
//     }
// }

fn orphan_rule() {
    println!("\n{:=>80}", "");
    println!("orphan_rule()\n");

    let bag: Vec<u32> = vec![1, 2, 3];
    println!("{}", bag.describe());
}
