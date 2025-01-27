use market::Market;


#[tokio::main]
async fn main() {
    let market = Market::new(); // Создание нового рынка | Создание также нового движка (пустой виртуальный бд)
    
    // Создание продавца
    let seller = market.create_seller("Laefye".to_string()).await.unwrap();
    // SellerContext - это структура, которая содержит в себе движок, продавца и покупателя
    // хранение этих полей необходимо для того, чтобы в дальнейшем можно было обращаться к ним через seller


    // Создание товара
    // Создание товара происходит через продавца, так как продавец является владельцем товара
    // Также good имеет информацию о том кто щас просматривает этот товар (viewer)
    let good = seller.create_good("Diamond Armor Set".to_string(), 32).await.unwrap();
    println!("Good: {:?}", good);

    // Получение товара
    // в данном случае мы получаем товар по его id из market, поэтому зритель (viewer) не указан
    let good = market.get_good(good.get_dto().id).await.unwrap().unwrap();
    println!("Good: {:?}", good);

    // Получение товара через продавца
    let good = seller.get_good(good.get_dto().id).await.unwrap().unwrap();
    println!("Good: {:?}", good);

    good.delete_good().await.unwrap();
}
