尽量从需要理解的顺序去记录


## 1. 从一个函数的传入
传入当中有几个直观的需求：一个是函数的定义，然后可以找到对应函数参数的类型，而这些类型是已经被存储和统计的。

```rust
fn some_system(a: Res<SomeComponentData>) {}
fn some_system_2(a: Res<SomeComponentData,SomeComponentData2>) {}
```

这样然后可以将对应的SomeComponentData那里去获取内容。

## 2. 然后他们使用TypeId去作为HashMap的索引
```rust
HashMap::new::<TypeId, Box<Dyn Any>>()
```
然后里面的Box<Dyn Any>则是为了获取任何未知的内容，且用智能指针包裹。

## 3. 如何从impl下去慢条斯理地理解好所有的trait的设计
从一开始的教程，是使用
```rust
impl<F: FnMut()> System<()> for F {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        (self)()
    }
}
```
以上的解释：创建一些任意泛型下为FnMut的无参数类型去实施System trait的行为。
然后从拓展参数量下，改写为
```rust 
impl<F: FnMut(T1), T1: 'static> System<(T1,)> for F {}
impl<F: FnMut(T1, T2), T1: 'static, T2: 'static> System<(T1, T2)> for F {}
```
来满足在接纳多个参数量下，在各种参数从类型ID去找对应的数据内容。

### 进而改写，对参数的抽象。
比如之前的
```rust
// type T = Input
trait System<Input> {}
/// 到
trait System{}
// 从原本的 
for F {} 
// 转到了 
for FunctionSystem<(), F> {}
```
也就是说，以前的System的trait是固定了fn run，而run的关于参数的类型从FunctionSystem的签名中代入，现在还是不是非常容易理解。

## 那么从这里去展开。
```rust
// 从用户的角度看到的是
fn my_system(a:Res<Running>){}
app.add_system(my_system);
```
我在add_system去拣选my_system函数中的参数里面的类型
a:Res<Running>,那么我可以获取 Res<Running>的泛型，
我可以获取其之前Stored的东西，比如我已经设置了
```rust
#[Component]
struct Running{
    speed: i32
}
```
这样的东西。
这个链路简单的理解目前是这样。
然后回归到上面说的，进而我们去统一地表达system的函数，那么我们去定义一个trait。如果在typescript或者任何面向对象的语言中，可能容易去定义接口或者一个class去保存它的typeid和参数的类型乃至数值或者字符串。

由于rust可以随时为任意类型去添加新的trait的行为实施。
乃至对于任何一个类型我们都会实施对于system的行为执行。
所有会产生了关于 impl System for F {} 的东西需求。
也就是之前说的
```rust
impl<T> System for F  
where T: FnMut<()>
{}
```
### 然后为什么会发生了关于FunctionSystem的内容呢？
文中提及到这会永久地从资源库中移除资源。在我们的ECS系统中如果有一个system拿出来了就还不回去了，别的system就无法再次获取也就不合理了。
原因是我们现在使用的Box<dyn Any>的，Box的拿出来使用，是会转移所有权的。那么我们进而对拿出来的东西进一步作处理了。
所有会有关于 ErasedSystem 的trait的定义了。并且就会被转移到一个新的 hashmap 中，而且基本跟前面保持原型。

```rust 
trait ErasedSystem {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>);
}

impl<S: System<I>, I> ErasedSystem for S {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        <Self as System<I>>::run(self, resources);
    }
}
```
然后产生了一个关于I无法确定是什么类型的错误。
这里很懵了，先看一下原本的关于System<I>的时候，是什么样的原来的代码。
```rust
// 从一开始的
trait System<Input>{}
// 然后到
trait System{}
```
而这个 缩减了system的泛型，是考虑到了这I类型的定向问题，所以才改为了trait System 的，也就是我们就是因为 System<I> 的存在，而把路走绝了，我们不能使用泛型去特质化System了。我们需要将System的数据区别出来了，System的语义仅仅在行为的执行特质上，而System的具体内容和执行方式都存都了一个新的类型中。所有这就产生一个FunctionSystem的东西。

首先原本的 F: FnMut<()> 和 F: FnMut<T1,T2> 这几个对一些接下来定义的类型的泛型约束，在实施system的拓展时，在这一步就统一一下这个多样化的入口，FnMut(T),FnMut(T,U)这些统一为一个FunctionSystem<..,F>来包裹，而这个F，就是原本需要定义的类型本身，作为这个FunctionSystem去接待定义进来的F服务，然后加以包装去使用。


## 在第一个可运行的test中
更需要整理的是system从多种需要用到的工具中转换到了多种的类型，
类型和类型之间的交互，如何在代码层面实现出来。

### 仍需要保持回顾
从一个单function下，记录了对应了函数中的参数从TypeId用HashMap记录它的FnMut内容
```rs
pub fn add_system<I, S: System + 'static>(&mut self, system: impl IntoSystem<I, System = S>) {
    self.systems.push(Box::new(system.into_system()));
}
```
这个函数中，参数trait对象下的self，和关于从IntoSystem中获取对应了关于I,和System=S的system的处理。
然后对于I，System=S, I是从最外部传入，先放一放讨论，然后到System=S,里面的代码是
```rs
pub trait IntoSystem<Input> {
    // 这里是定义一个关联类型System
    type System: System;
    // Input看作普通泛型 T。
    fn into_system(self) -> Self::System;
}
impl<F: FnMut()> IntoSystem<()> for F {
    type System = FunctionSystem<(), Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
impl<F: FnMut(T1), T1: 'static> IntoSystem<(T1,)> for F {
    type System = FunctionSystem<(T1,), Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
```
（我在代码块的注释里说明）

其实我们可以主要去看看T1的走向。
从各种的 impl 来看，关于F的是概括为都是FnMut(..)的特征拓展实施。
从中包括了关于 IntoSystem, 和原本就是F的，但是包裹了新的一层是 FunctionSystem<Input,F> 内接的F。
其实道理还是一样。
因为为了表达出参数 Input 的与 F 的关系,用一个结构体，
第一个存为自身的一个类型FunctionSystem，然后用PhantomData解释为Input泛型与FunctionSystem的关系。

回归到原始想要的东西
```rs
impl System for F {}
```
这里的F意思是说我写一些关于像System的函数出来，那么我的F可以解析出关于我函数的参数出来，
所以可以这样写写
```rs
impl Sysmte for Function {} 
```
大概是[./src/analyics/mod.rs](./src/analyics/mod.rs)的意思。

那么从操作语义上面去看，可以说原本的一个Function类型的,
而且有Fn,FnMut, 等的trait, 所以似乎就是为了模拟子类型去包裹一下对应这个函数的一些信息，phantomdata是为了满足编译器编译的。（补充：还为了标记类型关系，确保是Send/Sync）

所以务必在有一个全面的理解phantomdata下才能继续对类型系统的操作延伸性。
其本质上说明的是，子类型与函数之间的协变/逆协变关系。