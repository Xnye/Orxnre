namespace Orxnre;

public class Save
{
    public List<List<int[]>> Map;
    public int Money;
    public int TotalMoney;
    public int PX;
    public int PY;
    public readonly Role PRole;

    public Save(List<List<int[]>>? map = null, int? money = null, int? totalMoney = null, int? pX = null, int? pY = null, Role? pRole = null)
    {
        Map = map ?? new List<List<int[]>>();
        Money = money ?? 0;
        TotalMoney = totalMoney ?? 0;
        PX = pX ?? 0;
        PY = pY ?? 0;
        PRole = pRole ?? new(1, "玩家", 500, 500, 33, 100);
    }
}
