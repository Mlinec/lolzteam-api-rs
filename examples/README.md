# Examples

Примеры использования библиотеки lolzteam-api-rs.

## Структура

```
examples/
├── general/          # Общие примеры
│   ├── basic.rs              # Базовое использование
│   ├── error_handling.rs     # Обработка ошибок
│   ├── proxy.rs              # Использование прокси
│   └── separate_proxies.rs   # Раздельные прокси для форума и маркета
│
├── forum/            # Примеры Forum API
│   ├── activity.rs           # Активность пользователя
│   ├── categories.rs         # Категории форума
│   ├── chatbox.rs            # Чатбокс
│   ├── conversations.rs      # Личные сообщения
│   ├── misc.rs               # Разное (теги, навигация)
│   ├── notifications.rs      # Уведомления
│   ├── posts.rs              # Посты
│   ├── profile_posts.rs      # Посты на стене
│   ├── search.rs             # Поиск
│   ├── threads.rs            # Треды
│   └── users.rs              # Пользователи
│
└── market/           # Примеры Market API
    ├── arbitrage.rs          # Арбитраж (споры)
    ├── batch_operations.rs   # Пакетные операции
    ├── categories.rs         # Категории маркета
    ├── item.rs               # Информация о лоте
    ├── managing.rs           # Управление лотами
    ├── monitor.rs            # Мониторинг новых лотов
    ├── orders.rs             # Заказы
    ├── payments.rs           # Платежи и баланс
    ├── profile.rs            # Профиль продавца
    ├── purchasing.rs         # Покупка аккаунтов
    └── search.rs             # Поиск по маркету
```

## Запуск

### General
```bash
cargo run --example general/basic -- YOUR_TOKEN
cargo run --example general/error_handling -- YOUR_TOKEN
cargo run --example general/proxy -- YOUR_TOKEN
cargo run --example general/separate_proxies -- YOUR_TOKEN
```

### Forum
```bash
cargo run --example forum/activity -- YOUR_TOKEN [USER_ID]
cargo run --example forum/categories -- YOUR_TOKEN
cargo run --example forum/chatbox -- YOUR_TOKEN
cargo run --example forum/conversations -- YOUR_TOKEN
cargo run --example forum/misc -- YOUR_TOKEN
cargo run --example forum/notifications -- YOUR_TOKEN
cargo run --example forum/posts -- YOUR_TOKEN
cargo run --example forum/profile_posts -- YOUR_TOKEN USER_ID
cargo run --example forum/search -- YOUR_TOKEN
cargo run --example forum/threads -- YOUR_TOKEN
cargo run --example forum/users -- YOUR_TOKEN
```

### Market
```bash
cargo run --example market/arbitrage -- YOUR_TOKEN
cargo run --example market/batch_operations -- YOUR_TOKEN
cargo run --example market/categories -- YOUR_TOKEN
cargo run --example market/item -- YOUR_TOKEN ITEM_ID
cargo run --example market/managing -- YOUR_TOKEN ITEM_ID
cargo run --example market/monitor -- YOUR_TOKEN
cargo run --example market/orders -- YOUR_TOKEN
cargo run --example market/payments -- YOUR_TOKEN
cargo run --example market/profile -- YOUR_TOKEN USER_ID
cargo run --example market/purchasing -- YOUR_TOKEN ITEM_ID
cargo run --example market/search -- YOUR_TOKEN
```

## Примечания

- Примеры с операциями записи (создание постов, покупка, изменение цен) закомментированы или выводят предупреждения
- Для реальных операций раскомментируйте нужный код
- Некоторые примеры требуют дополнительных параметров (USER_ID, ITEM_ID)
