namespace Orxnre;

public class Role
{
    public int Type; // 类别

    public string Name; // 名称

    public int Hp; // 生命值
    public int HpMax; // 最大生命值
    public int Attack; // 伤害
    public int Mp; // 魔力值

    public bool IsAlive()
    {
        return Hp > 0;
    }

    public Role(int type = -1, string name = "", int hp = 0, int hpmax = 0, int attack = 0, int mp = 0)
    {
        Type = type;
        Name = name;
        Hp = hp;
        HpMax = hpmax;
        Attack = attack;
        Mp = mp;
    }
}

public class BattleStatus
{
    public int Win = -1; // 0=攻击者获胜, 1=受攻击者获胜
    public Role Loser = new();
    public Role Winner = new();
    public List<Role> LogRole = new(); // 战斗信息 - Role
    public List<int> LogInt = new(); // 战斗信息 - 伤害
}
public class Battle
{
    private readonly Random _battleRandom = new();
    // 普通攻击
    public BattleStatus Attack(Role attacker, Role receiver, int noise = 2)
    {
        var battleLog = new BattleStatus();
        while (true)
        {
            // 攻击者回合
            if (attacker.IsAlive())
            {
                var deHp = attacker.Attack + _battleRandom.Next(-noise, noise);
                receiver.Hp -= deHp;
                battleLog.LogRole.Add(attacker);
                battleLog.LogInt.Add(deHp);
            }
            // 受攻击者获胜
            else { battleLog.Win = 1; battleLog.Loser = attacker; battleLog.Winner = receiver; break; }
            
            // 被攻击者回合
            if (receiver.IsAlive())
            {
                var deHp = receiver.Attack + _battleRandom.Next(-noise, noise);
                attacker.Hp -= deHp;
                battleLog.LogRole.Add(receiver);
                battleLog.LogInt.Add(deHp);
            }
            // 攻击者获胜
            else { battleLog.Win = 0; battleLog.Loser = receiver; battleLog.Winner = attacker; break; }
        }

        return battleLog;
    }
}