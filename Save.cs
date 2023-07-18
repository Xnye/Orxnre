namespace Orxnre;

public class Save
{
    public List<List<int[]>> Map = new();
    public int Money;
    public int TotalMoney = 0;
    public int PX = 0;
    public int PY = 0;
    public readonly Role PRole = new(1, "玩家", 500, 500, 33, 100);

    public static readonly Save defaultSave = new();
}
