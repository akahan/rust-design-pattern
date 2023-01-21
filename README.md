<div align="center">
  <img src="/rust-logo.png" height="128" alt="logo">
  <h1 align="center">
    Rust Design Patterns
  </h1>
</div>

## Creational Patterns

|                       Pattern                       | Description                                                                         | Status |
|:---------------------------------------------------:|:------------------------------------------------------------------------------------|:------:|
|      [Factory Method](/creational/factory.rs)       | Defers instantiation of an object to a specialized function for creating instances. |   ✔    |
| [Abstract Factory](/creational/abstract_factory.rs) | Provides an interface for creating families of related objects.                     |   ✔    |
|   [Simple Factory](/creational/simple_factory.rs)   | Creates objects without specifying the exact class to create.                       |   ✔    |
|          [Builder](/creational/builder.rs)          | Builds a complex object using simple objects.                                       |   ✔    |
|        [Singleton](/creational/singleton.rs)        | Restricts instantiation of a type to one object.                                    |   ✔    |
|        [Prototype](/creational/singleton.rs)        | Creates objects by cloning an existing object.                                      |   ✔    |


## Behavioral Patterns

|                              Pattern                              | Description                                                                                     | Status |
|:-----------------------------------------------------------------:|:------------------------------------------------------------------------------------------------|:------:|
|                [Strategy](/behavioral/strategy.rs)                | Enables an algorithm's behavior to be selected at runtime.                                      |   ✔    |
|                   [State](/behavioral/state.rs)                   | Encapsulates varying behavior for the same object based on its internal state.                  |   ✔    |
|                 [Command](/behavioral/command.rs)                 | Converts requests or simple operations into objects.                                            |   ✔    |
|                [Iterator](/behavioral/iterator.rs)                | Lets you traverse elements of a collection without exposing its underlying representation.      |   ✔    |
|                [Observer](/behavioral/observer.rs)                | Allows one objects to notify other objects about changes in their state.                        |   ✔    |
| [Chain of Responsibility](/behavioral/chain_of_responsibility.rs) | Avoids coupling a sender to receiver by giving more than object a chance to handle the request. |   ✔    |




## Structural Patterns

|                Pattern                | Description                                                                              | Status |
|:-------------------------------------:|:-----------------------------------------------------------------------------------------|:------:|
|   [Adapter](/structural/adapter.rs)   | Allows objects with incompatible interfaces to collaborate.                              |   ✔    |
| [Decorator](/structural/decorator.rs) | Adds behavior to an object, statically or dynamically.                                   |   ✔    |
|     [Proxy](/structural/proxy.rs)     | Provides a surrogate for an object to control it's actions.                              |   ✔    |
| [Flyweight](structural/flyweight.rs)  | Reduces the cost of creating and manipulating a large number of similar objects.         |   ✔    |
|    [Bridge](structural/bridge.rs)     | Decouples an abstraction from its implementation so that the two can vary independently. |   ✔    |
| [Composite](structural/composite.rs)  | Composes zero-or-more similar objects so that they can be manipulated as one object.     |   ✔    |
|    [Facade](structural/facade.rs)     | Provides a simplified interface to a large body of code.                                 |   ✔    |
