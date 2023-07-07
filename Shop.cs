namespace Orxnre;
internal class Shop
{
    public class shopItem
    {
        public required Item.Single Goods;
        public required int Price;
        public int Remains = -1;
    }
    public static shopItem[] shopInventory =
    {
        new shopItem() {Goods = Item.List[1], Price = 100},
        new shopItem() {Goods = Item.List[2], Price = 100}
    };
}
