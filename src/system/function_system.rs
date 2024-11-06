use std::marker::PhantomData;

pub struct FunctionSystem<Input, F> {
    pub f: F,
    // we need a marker because otherwise we're not using `Input`.
    // fn() -> Input is chosen because just using Input would not be `Send` + `Sync`,
    // but the fnptr is always `Send` + `Sync`.
    //
    // Also, this way Input is covariant, but that's not super relevant since we can only deal with
    // static parameters here anyway so there's no subtyping. More info here:
    // https://doc.rust-lang.org/nomicon/subtyping.html
    pub(crate) marker: PhantomData<fn() -> Input>,
}


// 这里的phantomdata，是为了可以存储关于这个类型的信息，
// 而不实际存储这个类型里面的值的。
// 1.标记类型
// 2.所有权和生命周期，帮助编译器理解这个类型的所有权和生命周期关系信息。

// 为什么是 fn() -> Input 呢
// 标记 Input 类型，使编译器知道 FunctionSystem 与 Input 类型相关联。
// 确保 FunctionSystem 结构体是 Send 和 Sync 的，因为函数指针类型总是 Send 和 Sync 的。
// 确保 Input 类型是协变的，避免类型系统中的问题。

// 似乎也有可以直接翻译的可能，
// 是一个fn()->Input的幽灵数据的意思
// 保留了对于零成本和隐含的Send/Sync trait的impl需求下
// 在编译器理解语义下完成工作。