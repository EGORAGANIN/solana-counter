## Смарт-контракт счетчика
* Смарт-контракт поддерживает операции инкремента, декремента, сброса и обновления настроек.  
Для каждого пользователя создается отдельный счетчик.
Изменение настроек для операций инкремента и декремента может только проводить администратор.
Настройки глобальные для всех пользователей.
* Успешно развернут в testnet https://api.testnet.solana.com
Результаты работы можно в https://explorer.solana.com/?cluster=testnet для указанных ниже аккаунтов
* `counter/smart-contract/src` - исходный код контракта.
Код контракта покрыт unit тестами, расположенными в файлах модулей
* `counter/smart-contract/tests` - функциональные тесты

***

## Rust RPC клиент
* RPC клиент для проверки работы смарт-контракта непосредственное в живом окружении.
Производит инициализацию контракта и все основыне операции.
Результаты каждого действия выводит в консоль, для наглядности.
* `counter/rpc-client` - исходный код RPC клиента

***

## Аккаунты
* `counter/keypair` - ключи для аккаунтов

```
2wY7hT8TJhFpQqQJ5PGSed76vEgGNeQ11y1PvPsLUcS4 - admin.json - аккаунт администратора 
4UPHhQxnJrsmLE5w1qLencgCCttYiPswdaRRpQ9xwG5Z - user.json - аккаунт пользователя
7eWFSioVjHdJjbobEZu6hn5QLhmjWSv7qLMyCuzamYCG - program.json - аккаунт смарт-контракта
Ffav6rApgVYVogddJrLsccYwveUZCS8KJoM5TLW8T6CH - публичный ключ аккаунта счетчика для пользователя из примера PDA
5KCTQH1ZLtbm3C9AmatRBt4roj6yjoVErS2xMkLAN3nA - публичный ключ аккаунта настроек для счетчика PDA
```

***

### Сборка и unit-тесты смарт-контракта
```
$ cd smart-contract
$ cargo build
$ cargo test
```

### Сборка и функциональные тесты смарт-контракта BPF формат.
```
$ cd smart-contract
$ cargo build-bpf
$ cargo test-bpf
```

### Запуск RPC клиента
```
$ cd rpc-client
Для testnet
$ cargo run https://api.testnet.solana.com/
Для localhost
$ cargo run 
```