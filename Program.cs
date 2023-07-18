namespace Orxnre;

using System.Security.Cryptography;
using System.Text;
using System;
using System.IO;
using Newtonsoft.Json;
using static Orxnre.Shop;

public abstract class Program
{
    private static bool _onPrint; // 打印判定
    private static StringBuilder _nextPrint = new(); // 缓冲区
    private static bool _enableColor; // 颜色开关

    public const string Version = "1.0β2"; // 版本
    
    public static void Main()
    {
        // 清屏
        static void Cs()
        {
            for (var y = Console.WindowTop; y < Console.WindowTop + Console.WindowHeight; y++) {
                    Console.SetCursorPosition(Console.WindowLeft, y);
                    Console.Write(new string(' ', Console.WindowWidth));
            }
            Console.SetCursorPosition(Console.WindowLeft, Console.WindowTop);
            _onPrint = false;
            _nextPrint = new StringBuilder();
        }
        // 输出缓存
        static void Pt()
        {
            if (_onPrint == false)
            {
                _onPrint = true;
                Console.Write(_nextPrint);
                _nextPrint = new StringBuilder();
            }
        }
        
        Random trueRandom = new Random(); // 随机生成器

        Battle battle = new(); // 实例化Battle类
        
        // 关闭颜色时的打印优化
        static void PrintMaster(int type, object t)
        {
            if (_enableColor)
            { 
                switch (type)
                {
                    case 0:
                        Console.Write(t);
                        break;
                    case 1:
                        Console.WriteLine(t);
                        break;
                    case 2:
                        Console.WriteLine();
                        break;
                }
            }
            else
            {
                string? addNextPrint = t.ToString();
                addNextPrint += type == 1 ? "\n" : "";
                _nextPrint.Append(addNextPrint);
            }
        }
        static void Read()
        {
            Pt();
            Console.Write(_nextPrint);
            Console.Read();
        }
        static ConsoleKeyInfo ReadK()
        {
            Pt();
            Console.Write(_nextPrint);
            return Console.ReadKey();
        }
        static string? ReadL()
        {
            Pt();
            Console.Write(_nextPrint);
            return Console.ReadLine();
        }
        
        // 定义print
        static void Print(object t) => PrintMaster(0, t);
        static void Printl(object t) => PrintMaster(1, t);
        static void Printr() => PrintMaster(2, "\n");

        //全局配置
        string path = Directory.GetCurrentDirectory();
        string pathConfig = path + "\\config.txt";
        bool pathExist = File.Exists(pathConfig);

        while (true)
        {
            ////// 游戏相关 //////
            byte inOrxnre = 1;

            // 提示文本初始化
            string text = "";

            // 初始化地图和敌人列表
            static List<List<int[]>> NewMapStr()
            {
                List<List<int[]>> emptyMapStr = new();
                for (byte y = 0; y <= 9; y++)
                {
                    List<int[]> mapStrLine = new();
                    for (byte x = 0; x <= 9; x++) { mapStrLine.Add(new int[3]); }
                    emptyMapStr.Add(mapStrLine);
                }
                return emptyMapStr;
            }
            List<List<int[]>> mapStr = NewMapStr();

            static List<List<Role>> NewRole()
            {
                List<List<Role>> emptyRole = new();
                for (byte y = 0; y <= 9; y++)
                {
                    List<Role> mapRoleLine = new();
                    for (byte x = 0; x <= 9; x++) { mapRoleLine.Add(new Role()); }
                    emptyRole.Add(mapRoleLine);
                }
                return emptyRole;
            }
            List<List<Role>> mapRole = NewRole();

            // 初始化玩家位置和信息
            var pX = 0; var nextPx = pX;
            var pY = 0; var nextPy = pY;
            var money = 0; var totalMoney = 0;
            Role pRole = new(1, "玩家", 500, 500, 33, 100);
            // 打印信息初始化
            var currentColor = ConsoleColor.White;
            string currentStr;
            int currentNum;

            //def: 控制台颜色和打印相关
            void Color(ConsoleColor cf = ConsoleColor.White, ConsoleColor cb = ConsoleColor.Black)
            {
                if (_enableColor)
                {
                    Console.ForegroundColor = cf;
                    Console.BackgroundColor = cb;
                }
            }
            Color();

            // UI相关
            void DrawMenuTitle(bool clear = true, int way = 0)
            {
                if (clear)
                {
                    Cs();
                }

                switch (way)
                {
                    case 0:
                        Printl($"│ Orxnre {Version}\n└──────────────┐");
                        break;
                    case 1:
                        Printl($"Orxnre {Version}\n");
                        break;
                }
            }
            int Select(List<string> selections, string question = "", bool clear = true, int way = 0)
            {
                var selectionsCount = selections.Count;
                var selected = 0;
                while (true)
                {
                    DrawMenuTitle(clear: clear, way: way);
                    Print(question == "" ? question : question + "\n");
                    for (var i = 0; i < selectionsCount; i++)
                    {
                        switch (way)
                        {
                            case 1:
                                Printl(i == selected ? $" ├─[ {selections[i]} ]   " : $" │   {selections[i]}     ");
                                break;
                            case 0:
                                Printl(i == selected ? $"  [ {selections[i]} ]─┤ " : $"    {selections[i]}   │ ");
                                break;
                        }
                    }

                    var ioInputKey = ReadK().Key;
                    switch (ioInputKey)
                    {
                        case ConsoleKey.UpArrow:
                            if (selected != 0)
                            {
                                selected--;
                            }
                            break;
                        case ConsoleKey.DownArrow:
                            if (selected != selectionsCount - 1)
                            {
                                selected++;
                            }
                            break;
                        case ConsoleKey.Enter:
                            return selected;
                    }
                }
            }

            // def: 地图填充模块
            void MapFill(int type, int x1, int x2, int y1, int y2)
            {
                for (var y = y1; y <= y2; y++)
                {
                    for (var x = x1; x <= x2; x++)
                    {
                        try
                        {
                            mapStr[y][x][1] = type;
                        }
                        catch
                        {
                        }
                    }
                }
            }

            // def: 设置金币模块
            void MapMoney(int amount, int x, int y)
            {
                mapStr[y][x][0] = 1;
                mapStr[y][x][2] = amount;
            }

            // def: Role信息显示器
            string RoleAsciiInfoHealthbar(int hp, int max, int len = 20)
            {
                var linePart = new decimal(len * hp / max);
                return new string('=', (int)Math.Round(linePart, 0, MidpointRounding.AwayFromZero));
            }
            string RoleAsciiInfo(Role who)
            {
                //(格式示例)
                //┌────────────────────────┐
                //│ Enemy    181284/350000 │
                //│ [====================] │
                //└────────────────────────┘
                string result = "";

                string line = RoleAsciiInfoHealthbar(who.Hp, who.HpMax);
                string hpAndMaxhp = $"{who.Hp}/{who.HpMax}";

                result += "┌────────────────────────┐\n";
                result += $"│ Enemy    {hpAndMaxhp,13} │\n";
                result += $"│ [{line,-20}] │\n";
                result += "└────────────────────────┘";

                return result;
            }

            ////// 初始化就绪, 进入标题界面 //////
            switch (Select (new List<string> {"开始游戏","读取存档", "退出程序"}))
            {
                case 0:
                    Random inputRandom = new Random();
                    MD5 seedHash = MD5.Create();
                    DrawMenuTitle(way: 1);
                    Print("请输入种子 (随机请留空):");
                    string? seedInput = ReadL();
                    if (seedInput != null)
                    {
                        seedHash.ComputeHash(Encoding.UTF8.GetBytes(seedInput));
                        inputRandom = new Random(seedHash.GetHashCode());
                    }

                    void MapInit(Random rdGen)
                    {
                        // 生成敌人
                        for (var i = 0; i < 3; i++)
                        {
                            mapRole[rdGen.Next(0, 9)][rdGen.Next(0, 9)] = new Role(2, "敌人", 100, 100, 140, 100);
                        }
                        // 全图填充土
                        MapFill(1, 0, 9, 0, 9);
                        // 生成石
                        for (byte i = 0; i < 3; i++)
                        {
                            int setLx = rdGen.Next(0, 9); int setLy = rdGen.Next(0, 9);
                            int setLxf = setLx + rdGen.Next(0, 2); int setLyf = setLy + rdGen.Next(0, 2);
                            MapFill(3, setLx, setLxf, setLy, setLyf);
                        }
                        // 生成草
                        for (byte i = 0; i < 3; i++)
                        {
                            int setLx = rdGen.Next(0, 9); int setLy = rdGen.Next(0, 9);
                            int setLxf = setLx + rdGen.Next(0, 3); int setLyf = setLy + rdGen.Next(0, 3);
                            MapFill(2, setLx, setLxf, setLy, setLyf);
                        }

                        // 随机设置金币
                        for (int i = 0; i < 10; i++)
                        {

                            MapMoney(rdGen.Next(200, 300), rdGen.Next(0, 9), rdGen.Next(0, 9));
                        }
                    }
                    MapInit(inputRandom);
                    break;
                case 1:
                    if (pathExist)
                    {
                        using (var configfile = new StreamReader(File.OpenRead(pathConfig)))
                        {
                            var savesByte = configfile.ReadToEnd();
                            var readSave = JsonConvert.DeserializeObject<Save>(savesByte);
                            if (readSave != null)
                            {
                                mapStr = readSave.Map;
                                money = readSave.Money;
                                pX = readSave.PX;
                                pY = readSave.PY;
                            }
                        }
                    }
                    else
                    {
                        Printl("文件不存在, 将不读取直接开始游戏.");
                    }
                    Printl("点击任意键继续...");
                    Read();
                    break;
                case 2:
                    Environment.Exit(0);
                    break;
            }
   
            //// 游戏开始
            while (inOrxnre == 1)
            {
                // 绘制标题
                void DrawIngameTitle(string position = "")
                {
                    Cs();
                    Printl($"Orxnre {Version}{(position=="" ? "" : " |「" + position + "」")}");
                    Print($"$ {money} ");
                    Print($"[{RoleAsciiInfoHealthbar(pRole.Hp, pRole.HpMax, 15),-15}] {pRole.Hp}/{pRole.HpMax}");
                    Printr();
                }
                DrawIngameTitle();
                
                // 绘制玩家位置和地图
                for (var y = 0; y <= 9; y++)
                {
                    for (var x = 0; x <= 9; x++)
                    {
                        if (y == pY && x == pX)
                        {
                            Print("您 ");
                        }
                        else
                        {
                            // 判断该位置的编号是否超出范围以及是否有敌人并打印
                            currentNum = mapStr[y][x][1];
                            if (mapRole[y][x].Type != -1)
                            {
                                currentStr = "!!";
                            }
                            else 
                            {
                                if (currentNum >= 0 && currentNum < BlockInfo.List.Length)
                                {
                                    currentStr = BlockInfo.List[currentNum].Title;
                                    currentColor = BlockInfo.List[currentNum].Color;
                                }
                                else
                                {
                                    currentStr = "??";
                                }
                            }
                            Color(currentColor);
                            Print(currentStr + " ");
                            Color();
                        }
                    }
                    Printr();
                }
                // 打印提示
                currentNum = mapStr[pY][pX][1];
                if (currentNum >= 0 && currentNum < BlockInfo.List.Length)
                {
                    currentStr = BlockInfo.List[currentNum].Title;
                    currentColor = BlockInfo.List[currentNum].Color;
                }
                else
                {
                    currentStr = "??";
                }
                // 打印该位置地图块信息和按键提示
                Color(ConsoleColor.DarkGray);
                Print("\n[");
                Color(currentColor);
                Print($"{currentStr}");
                Color(ConsoleColor.DarkGray);
                Print("]");
                Print($" {BlockInfo.List[currentNum].Explain}\n");
                Printl("[E-探索] [Q-商店] [C-颜色]");
                Color();
                // 清空文本并打印信息
                Printl(text);
                text = "";

                var saves = new Save
                {
                    Map = mapStr,
                    Money = money,
                    PX = pX,
                    PY = pY
                };
                var savesJson = JsonConvert.SerializeObject(saves);

                // 等待并获取输入
                var ioInput = ReadK().Key;
                switch (ioInput)
                {
                    // WASD 玩家移动
                    case ConsoleKey.W:
                        nextPy = pY - 1; break;
                    case ConsoleKey.S:
                        nextPy = pY + 1; break;
                    case ConsoleKey.A:
                        nextPx = pX - 1; break;
                    case ConsoleKey.D:
                        nextPx = pX + 1; break;
                    // E 寻找宝藏
                    case ConsoleKey.E:
                        // 挖掘理论等待时间 (450ms)
                        for (int i = 0; i <= 10; i++)
                        {
                            Console.Write($"\r搜索中 [{RoleAsciiInfoHealthbar(i, 10, 10), -10}]");
                            Thread.Sleep(45);
                        }
                        // 判断该处是否有宝箱
                        if (mapStr[pY][pX][0] == 1)
                        {
                            var moneyAdding = mapStr[pY][pX][2];
                            money += moneyAdding;
                            totalMoney += moneyAdding;
                            text = $"找到了宝藏! (+{moneyAdding})";
                            mapStr[pY][pX][2] = 0;
                            mapStr[pY][pX][0] = 0;
                        }
                        else
                        {
                            text = "空空如也";
                        }
                        break;
                    // C 颜色开关
                    case ConsoleKey.C:
                        Color();
                        _enableColor = !_enableColor;
                        text = _enableColor ? "已开启颜色" : "已关闭颜色";
                        break;
                    // L 保存进度
                    case ConsoleKey.L:
                        savesJson = JsonConvert.SerializeObject(saves);

                        bool pathCreating;
                        if (pathExist){
                            pathCreating = true;
                            text = pathConfig;
                        }
                        else{
                            text = $"文件不存在。已在目录下创建: {pathConfig}";
                            pathCreating = true;
                        }
                        if (pathCreating)
                        {
                            savesJson = JsonConvert.SerializeObject(saves);
                            using var configfile = File.Create(pathConfig);
                            configfile.Write(Encoding.Default.GetBytes(savesJson));
                        }
                        break;

                    // Q 打开商店
                    case ConsoleKey.Q:
                        int selected = 0;
                        int nextSelect = 0;
                        bool inShop = true;
                        while (inShop)
                        {
                            // 绘制页面
                            DrawIngameTitle("商店");
                            Printr();
                            for (int i = 0; i < shopInventory.Length; i++)
                            {
                                shopItem item = shopInventory[i];
                                Printl($" {(selected == i ? ">" : " ")} {item.Goods.Title} - ${item.Price} {(selected == i ? "<" : " ")}");
                            }
                            Printl("[Q-退出] | [↑↓-选择] [Enter-确认]");
                            // 读取按键
                            ConsoleKey selectKey = ReadK().Key;
                            if (selectKey == ConsoleKey.UpArrow)
                            {
                                nextSelect = selected - 1;
                            }
                            else if (selectKey == ConsoleKey.DownArrow)
                            {
                                nextSelect = selected + 1;
                            }
                            else if (selectKey == ConsoleKey.Q)
                            {
                                inShop = false;
                            }
                            else if (selectKey == ConsoleKey.Enter)
                            {
                                shopItem selectedItem = shopInventory[selected];
                                bool canAfford = money >= selectedItem.Price;
                                if (canAfford)
                                {
                                    switch (selectedItem.Goods.GetType().Name)
                                    {
                                        case "Effect":
                                            // 防止增加后的血量超过上限
                                            int addedHp = pRole.Hp + ((Item.Effect)selectedItem.Goods).Count;
                                            pRole.Hp = addedHp > pRole.HpMax ? pRole.HpMax : addedHp;
                                            break;
                                    }
                                    money -= selectedItem.Price;
                                }
                            }
                            // 防止下一项超出列表范围
                            selected = (nextSelect >= 0 && nextSelect < shopInventory.Length) ? nextSelect : selected;
                        }
                        break;
                }
                // 判断该次移动是否合法
                if (nextPx is > -1 and < 10 && nextPy is > -1 and < 10)
                {
                    // 目标位置有敌人阻挡
                    if (mapRole[nextPy][nextPx].Type == 2 && mapRole[nextPy][nextPx].IsAlive())
                    {
                        Role currentRole = mapRole[nextPy][nextPx];

                        if (Select(new List<string> { "进攻", "取消" }, $"要发起进攻吗?\n{RoleAsciiInfo(currentRole)}", true, 1) == 0)
                        {
                            BattleStatus blog = battle.Attack(pRole, currentRole); // battle log
                            //  循环输出对战日志
                            for (var i = 0; i < blog.LogInt.Count; i++)
                            {
                                Console.WriteLine($"{blog.LogRole[i].Name} 发动了攻击, 造成 {blog.LogInt[i]} 伤害.");
                                Thread.Sleep(120);
                            }
                            // 如果攻击对象生命值耗尽则在该处新建空角色
                            if (blog.Win == 0)
                            {
                                mapRole[nextPy][nextPx] = new Role();
                            }
                            Console.WriteLine($"{blog.Loser.Name} 战败了, 点击任意键继续...");
                            Console.ReadKey();
                        }
                        // 对战结束后保留在原地
                        nextPx = pX;
                        nextPy = pY;
                        if (pRole.IsAlive() == false) {
                            inOrxnre = 2;
                            break;
                        }
                    }
                    else
                    {
                        // 移动合法, 更改位置
                        pX = nextPx; pY = nextPy;
                    }
                }
                else
                {
                    // 移动不合法, 保持位置
                    nextPx = pX; nextPy = pY;
                }
            }
            switch (inOrxnre)
            {
                case 1:
                    Console.WriteLine("已退出游戏.");
                    
                    break; 
                case 2: // 玩家失败提示
                    DrawMenuTitle(way: 1);
                    Pt();
                    Console.WriteLine("あや...你的血条见底了!");
                    Console.WriteLine($"累计 $ {totalMoney}");
                    break;
            }
            Thread.Sleep(1000);
            Console.WriteLine("点击任意键继续...");
            Console.ReadKey();
        }
    }
}
