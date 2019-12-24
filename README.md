
`rpn-encode` -- принимает один аргумент командной строки, строку с выражением, записывает результат в input.txt.

`rpn-eval` -- читает из input.txt, пишет результат в stdout.

Собрать -- `cargo build --release`.

По дефолту используется float, для других типов данных можно скомпилировать с `--features=TYPE --no-default-features`, где `TYPE` может быть f32, f64, u32, u64.

Подробнее -- [здесь](https://www.linux.org.ru/forum/development/15422159)
