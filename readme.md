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