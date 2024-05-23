fn main() {
    use std::collections::HashMap;

// 通过类型推断，我们可以省略显式类型签名 (在本示例中为 `HashMap<&str, u8>`)。
    let mut player_stats = HashMap::new();

    fn random_stat_buff() -> u8 {
        // 实际上可以在这里返回一些随机值 - 现在让我们返回一些固定值
        42
    }

// 仅在键不存在时才插入
    player_stats.entry("health").or_insert(100);
    println!("{:?}", player_stats);

// 仅当一个键不存在时，才使用提供新值的函数插入该键
    player_stats.entry("defence").or_insert_with(random_stat_buff);
    println!("{:?}", player_stats);

// 更新键，以防止键可能未被设置
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();
    println!("{:?}", player_stats);
    player_stats.insert("mana", 42u8);
// 使用就地可变的在插入之前修改条目
    player_stats.entry("mana").and_modify(|mana| { *mana += 200 }).or_insert(100);
    println!("{:?}", player_stats.entry("mana"));
}