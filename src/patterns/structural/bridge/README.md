## 桥接模式

### 概述


### 类图
```mermaid
classDiagram
    class Abstract {
        
    }
    
    class RefinedAbstractA {
        
    }

    class RefinedAbstractB {

    }
    
    class Implementor {
        
    }
    
    class ConcreteImplementorA {
        
    }

    class ConcreteImplementorB {

    }
    
    Abstract <|-- RefinedAbstractA
    Abstract <|-- RefinedAbstractB
    Implementor <|.. ConcreteImplementorA
    Implementor <|.. ConcreteImplementorB
    Abstract o--> Implementor
    
```