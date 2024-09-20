struct Example(i32);

impl Drop for Example {
    // При уничтожении экземпляра, выводим его значение.
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

struct ExampleWrap(Example);

impl Drop for ExampleWrap {
    // Заменяем внутренний экземпляр Example на новый с нулевым значением
    // перед уничтожением, чтобы увидеть исходное значение.
    fn drop(&mut self) {
        let old = std::mem::replace(&mut self.0, Example(0));
        println!("wrapped: {}", old.0);
    }
}

fn main() {
    // Временный экземпляр, уничтожается сразу.
    Example(1);

    // Экземпляр будет уничтожен в конце блока.
    let _e2 = Example(2);

    // Еще один временный экземпляр.
    let _e3 = Example(3);

    // Экземпляр уничтожается сразу.
    let _ = Example(4);

    // Создаем и затем уничтожаем экземпляр внутри Option.
    let mut _e5;
    _e5 = Some(Example(5));
    _e5 = None;

    // Явное уничтожение экземпляра.
    let e6 = Example(6);
    drop(e6);

    // Предотвращаем автоматическое уничтожение, возможна утечка памяти.
    let e7 = Example(7);
    std::mem::forget(e7);

    // Уничтожаем вложенную структуру, вызывая дроп для обоих уровней.
    ExampleWrap(Example(8));
}