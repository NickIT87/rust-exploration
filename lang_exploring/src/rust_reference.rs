//! Краткий справочник по основным возможностям Rust.
//! Запустите `cargo run`, чтобы увидеть примеры в действии.

use std::collections::HashMap;
use std::thread;

// Структура объединяет связанные данные.
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    active: bool,
}

// Перечисление задаёт несколько вариантов одного типа.
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Text(String),
}

// Трейт описывает поведение, которое могут реализовать разные типы.
trait Describe {
    fn describe(&self) -> String;
}

// Реализуем трейт для структуры User.
impl Describe for User {
    fn describe(&self) -> String {
        format!("{} ({} лет)", self.name, self.age)
    }
}

// Дженерик позволяет функции работать с разными типами.
fn print_item<T: std::fmt::Debug>(item: &T) {
    println!("Элемент: {item:?}");
}

// Ссылка на часть строки живёт не дольше исходной строки.
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

// Result сообщает об успехе (Ok) или ошибке (Err).
fn parse_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    // ? возвращает ошибку из этой функции, если строка не является числом.
    let number = input.parse()?;
    Ok(number)
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("деление на ноль")
    } else {
        Ok(a / b)
    }
}

pub fn run() {
    // Переменные неизменяемы по умолчанию; mut разрешает менять значение.
    let language = "Rust";
    let mut count = 1;
    count += 1;
    println!("{language}: счётчик = {count}");

    // Типы можно указать явно; часто Rust выводит их из контекста.
    let whole: i32 = 42;
    let fraction = 3.5_f64;
    let is_ready = true;
    let pair = (whole, fraction);
    println!("Типы: {}, {}, {}, кортеж {:?}", whole, fraction, is_ready, pair);

    // Владение: String перемещается в новую переменную.
    let original = String::from("владение");
    let moved = original;
    println!("После перемещения значение доступно через moved: {moved}");
    // original здесь использовать нельзя: владение уже передано в moved.

    // Copy-типы (например, целые числа) при присваивании копируются.
    let number = 10;
    let number_copy = number;
    println!("Copy: {number} и {number_copy}");

    // Неизменяемое заимствование даёт временный доступ без передачи владения.
    let sentence = String::from("Rust учит управлять памятью");
    println!("Первое слово: {}", first_word(&sentence));
    println!("Исходная строка всё ещё доступна: {sentence}");

    // Изменяемое заимствование позволяет временно менять значение.
    let mut greeting = String::from("Привет");
    greeting.push_str(", Rust!");
    println!("Изменяемая строка: {greeting}");

    // Срез — ссылка на непрерывную часть коллекции или строки.
    let numbers = [10, 20, 30, 40];
    let middle = &numbers[1..3];
    println!("Срез массива: {middle:?}");

    // if — выражение: ветви могут возвращать значение.
    let parity = if number % 2 == 0 { "чётное" } else { "нечётное" };
    println!("{number} — {parity}");

    // Циклы: for проходит по диапазону, while выполняется пока условие истинно.
    for index in 0..3 {
        print!("{index} ");
    }
    println!();
    let mut countdown = 2;
    while countdown > 0 {
        print!("{countdown} ");
        countdown -= 1;
    }
    println!();

    // match проверяет варианты; _ обозначает все остальные случаи.
    let messages = [
        Message::Quit,
        Message::Move { x: 3, y: -1 },
        Message::Text(String::from("Привет")),
    ];
    for message in messages {
        match message {
            Message::Quit => println!("Выход"),
            Message::Move { x, y } => println!("Перемещение: x={x}, y={y}"),
            Message::Text(text) => println!("Сообщение: {text}"),
        }
    }

    // Option представляет значение, которое может отсутствовать.
    let maybe_name: Option<&str> = Some("Ада");
    let name = match maybe_name {
        Some(value) => value,
        None => "гость",
    };
    println!("Имя: {name}");

    // Vec — изменяемый по размеру список элементов одного типа.
    let mut scores = vec![10, 20, 30];
    scores.push(40);
    println!("Вектор: {scores:?}; второй элемент: {}", scores[1]);

    // HashMap хранит пары ключ-значение.
    let mut capitals = HashMap::new();
    capitals.insert("Франция", "Париж");
    capitals.insert("Япония", "Токио");
    println!("Столица Японии: {:?}", capitals.get("Япония"));

    // Структуры создают собственные типы с именованными полями.
    let user = User {
        name: String::from("Алиса"),
        age: 30,
        active: true,
    };
    println!("Пользователь: {user:?}; активен: {}", user.active);

    // Методы объявляются в impl и вызываются через точку.
    println!("Описание: {}", user.describe());

    // Дженерик-функция принимает любой тип, который реализует Debug.
    print_item(&scores);

    // Замыкание — функция, которую можно сохранить в переменную.
    let multiplier = 2;
    let double = |value: i32| value * multiplier;
    println!("Замыкание: {}", double(21));

    // Итераторы позволяют преобразовывать и фильтровать последовательности.
    let doubled_evens: Vec<i32> = scores
        .iter()
        .copied()
        .filter(|score| score % 20 == 0)
        .map(|score| score * 2)
        .collect();
    println!("Удвоенные числа, кратные 20: {doubled_evens:?}");

    // ? внутри parse_number передаёт ошибку вызывающему коду.
    println!("Разбор числа из строки: {:?}", parse_number("123"));

    // Result можно обработать явно через match.
    match divide(10.0, 2.0) {
        Ok(value) => println!("Результат деления: {value}"),
        Err(error) => println!("Ошибка: {error}"),
    }

    // Поток получает владение значением через move и возвращает результат через join.
    let worker = thread::spawn(move || {
        format!("Поток обработал число {number_copy}")
    });
    println!("Параллельное выполнение: {}", worker.join().unwrap());

    // Макросы вызываются с !; println! и vec! — стандартные макросы Rust.
    let labels = vec!["безопасность", "скорость", "контроль"];
    println!("Макрос vec!: {labels:?}");
}

/*
Rust: счётчик = 2
Типы: 42, 3.5, true, кортеж (42, 3.5)
После перемещения значение доступно через moved: владение
Copy: 10 и 10
Первое слово: Rust
Исходная строка всё ещё доступна: Rust учит управлять памятью
Изменяемая строка: Привет, Rust!
Срез массива: [20, 30]
10 — чётное
0 1 2 
2 1 
Выход
Перемещение: x=3, y=-1
Сообщение: Привет
Имя: Ада
Вектор: [10, 20, 30, 40]; второй элемент: 20
Столица Японии: Some("Токио")
Пользователь: User { name: "Алиса", age: 30, active: true }; активен: true
Описание: Алиса (30 лет)
Элемент: [10, 20, 30, 40]
Замыкание: 42
Удвоенные числа, кратные 20: [40, 80]
Разбор числа из строки: Ok(123)
Результат деления: 5
Параллельное выполнение: Поток обработал число 10
Макрос vec!: ["безопасность", "скорость", "контроль"]
*/