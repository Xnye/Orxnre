namespace Orxnre;

public abstract class Item
{
    public class Single
    {
        public required string Title;
        public int Amount = 1;
    }
    public class Effect: Single
    {
        public required int EffectID;
        public int Count = 0;
    }
    public class Weapon: Single
    {
        public required int Damage;
    }

    public static readonly Single[] List =
    {
        new Weapon() {Title = "Zero", Damage = 0},
        new Effect() {Title = "血量恢复-I", EffectID = 1, Count = 100},
        new Effect() {Title = "血量恢复-II", EffectID = 1, Count = 200}
    };
}
