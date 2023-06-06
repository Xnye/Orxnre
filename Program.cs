namespace Orxnre;

using System.Security.Cryptography;
using System.Text;
using System;
using System.IO;
using Newtonsoft.Json;

public abstract class Program
{
    private static bool _onPrint;
    private static StringBuilder _nextPrint = new();
    private static bool _enableColor;

    private const string Version = "1.0β1"; // 版本
    
    private class SaveInfo
    {
        public List<List<int[]>> Map = new();
        public int M;
        public int X;
        public int Y;
    }
    
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

        static void Pt()
        {
            if (_onPrint == false)
            {
                _onPrint = true;
                Console.Write(_nextPrint);
                _nextPrint = new StringBuilder();
            }
        }
        
        var trueRandom = new Random(); // 随机生成器

        Battle battle = new Battle(); // 实例化Battle类
        
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
                if (type == 1)
                {
                    addNextPrint += "\n";
                }
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

        //配置文件相关
        var path = Directory.GetCurrentDirectory();
        var pathConfig = path + "\\config.txt";
        var pathExist = File.Exists(pathConfig);

        var inOrxnre = true;
        
        // 提示文本初始化
        var text = "";
        var moneyText = "";

        // 初始化地图和敌人列表
        static List<List<int[]>> NewMapStr()
        {
            List<List<int[]>> emptyMapStr = new List<List<int[]>>();
            for (var i = 0; i <= 9; i++)
            {
                List<int[]> mapStrLine = new List<int[]>();
                for (var j = 0; j <= 9; j++)
                {
                    mapStrLine.Add(new int[3]);
                }
                emptyMapStr.Add(mapStrLine);
            }
            return emptyMapStr;
        }
        List<List<int[]>> mapStr = NewMapStr();
        
        static List<List<Role>> NewRole()
        {
            List<List<Role>> emptyRole = new List<List<Role>>();
            for (var i = 0; i <= 9; i++)
            {
                List<Role> mapRoleLine = new List<Role>();
                for (var j = 0; j <= 9; j++)
                {
                    mapRoleLine.Add(new Role());
                }
                emptyRole.Add(mapRoleLine);
            }
            return emptyRole;
        }
        List<List<Role>> mapRole = NewRole();
        
        // 初始化玩家位置和信息
        var pX = 0; var nextPx = pX;
        var pY = 0; var nextPy = pY;
        var money = 0;
        Role pRole = new Role(1, "玩家", 500, 20, 100);
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
        void DrawMenuTitle (bool clear = true, int way = 0)
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
        int Select (List<string> selections, string question = "", bool clear = true, int way = 0)
        {
            var selectionsCount = selections.Count;
            var selected = 0;
            while (true) {
                DrawMenuTitle(clear: clear, way: way);
                Print(question == "" ? question : question + "\n");
                for (var i = 0; i < selectionsCount; i++) {
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
                switch (ioInputKey) {
                    case ConsoleKey.UpArrow:
                        if (selected != 0) {
                            selected--;
                        }
                        break;
                    case ConsoleKey.DownArrow:
                        if (selected != selectionsCount - 1) {
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
            for (var i = x1; i <= x2; i++)
            {
                for (var j = y1; j <= y2; j++)
                {
                    try
                    {
                        mapStr[i][j][1] = type;
                    }
                    catch
                    {
                        // ignored
                    }
                }
            }
        }

        // def: 设置金币模块
        void MapMoney(int amount, int x, int y)
        {
            mapStr[x][y][0] = 1;
            mapStr[x][y][2] = amount;
        }

        string RoleAsciiInfo(Role who)
        {
            //┌──────────────────────┐
            //│Enemy          100/100│
            //│[====================]│
            //└──────────────────────┘
            string result = "";
            var hp = who.Hp;
            var attack = who.Attack;
            var mp = who.Mp;
            result += "------\n"; 
            return result;
        }

        while (true)
        {
            switch (Select (new List<string> {"开始游戏","读取存档", "退出程序"}))
            {
                case 0:
                    var inputRandom = new Random();
                    var seedHash = MD5.Create();
                    DrawMenuTitle(way: 1);
                    Print("请输入种子 (随机请留空):");
                    var seedInput = ReadL();
                    if (seedInput != null)
                    {
                        seedHash.ComputeHash(Encoding.UTF8.GetBytes(seedInput));
                        inputRandom = new Random(seedHash.GetHashCode());
                    }

                    void MapInit(Random rdGen)
                    {
                        // 生成敌人
                        mapRole[rdGen.Next(0, 9)][rdGen.Next(0, 9)] = new Role(2, "敌人", 100, 5, 100);
                        // 全图填充土
                        MapFill(1, 0, 9, 0, 9);
                        // 生成石
                        for (var i = 0; i < 3; i++)
                        {
                            var setLx = rdGen.Next(0, 9); var setLy = rdGen.Next(0, 9);
                            var setLxf = setLx + rdGen.Next(0, 2); var setLyf = setLy + rdGen.Next(0, 2);
                            MapFill(3, setLx, setLxf, setLy, setLyf);
                        }
                        // 生成草
                        for (var i = 0; i < 3; i++)
                        {
                            var setLx = rdGen.Next(0, 9); var setLy = rdGen.Next(0, 9);
                            var setLxf = setLx + rdGen.Next(0, 3); var setLyf = setLy + rdGen.Next(0, 3);
                            MapFill(2, setLx, setLxf, setLy, setLyf);
                        }

                        // 随机设置金币
                        for (var i = 0; i < 10; i++)
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
                            var readSave = JsonConvert.DeserializeObject<SaveInfo>(savesByte);
                            if (readSave != null)
                            {
                                mapStr = readSave.Map;
                                money = readSave.M;
                                pX = readSave.X;
                                pY = readSave.Y;
                            }
                        }
                    }
                    else
                    {
                        Printl("File not found.");
                    }
                    Printl("Press any key to continue...");
                    Read();
                    break;
                case 2:
                    Environment.Exit(0);
                    break;
            }
   
            //// 游戏开始
            while (inOrxnre)
            {
                // 绘制标题
                void DrawIngameTitle()
                {
                    Cs();
                    Printl($"Orxnre {Version}");
                    Print($"$ {money} ");
                    if (moneyText != "")
                    {
                        Color(ConsoleColor.Yellow);
                        Print($"{moneyText}");
                        Color();
                        moneyText = "";
                    }
                    Printr();
                }
                DrawIngameTitle();
                
                // 绘制玩家位置和地图
                for (var i = 0; i <= 9; i++)
                {
                    for (var j = 0; j <= 9; j++)
                    {
                        if (i == pY && j == pX)
                        {
                            Print("您 ");
                        }
                        else
                        {
                            // 判断该位置的编号是否超出范围以及是否有敌人并打印
                            currentNum = mapStr[j][i][1];
                            if (mapRole[j][i].Type == 2)
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
                currentNum = mapStr[pX][pY][1];
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
                Printl("[L-保存进度] | [E-探索] [Q-商店] [C-颜色]");
                Color();
                // 清空文本并打印信息
                Printl(text);
                text = "";

                var saves = new SaveInfo
                {
                    Map = mapStr,
                    M = money,
                    X = pX,
                    Y = pY
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
                        if (mapStr[pX][pY][0] == 1)
                        {
                            var moneyAdding = mapStr[pX][pY][2];
                            money += moneyAdding;
                            text = "找到了宝藏!";
                            moneyText = $"(+{moneyAdding})";
                            mapStr[pX][pY][2] = 0;
                            mapStr[pX][pY][0] = 0;
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
                            using (var configfile = File.Create(pathConfig))
                            {
                                configfile.Write(Encoding.Default.GetBytes(savesJson));
                            }
                        }
                        break;
                }
                // 判断该次移动是否合法
                if (-1 < nextPx && nextPx < 10 && -1 < nextPy && nextPy < 10)
                {
                    if (mapRole[nextPx][nextPy].Type == 2)
                    {
                        var currentRole = mapRole[nextPx][nextPy];

                        if (Select(new List<string> { "进攻", "取消" }, $"要发起进攻吗? TargetHP={currentRole.Hp}", true, 1) == 0)
                        {
                            var blog = battle.Attack(pRole, currentRole); // battle log
                            
                            for (int i = 0; i < blog.LogInt.Count; i++)
                            {
                                Console.WriteLine($"{blog.LogRole[i].Name} 发动了攻击, 造成 {blog.LogInt[i]} 伤害.");
                                Thread.Sleep(150);
                            }
                            Console.WriteLine("点击任意键继续");
                            Console.ReadKey();
                        }
                        nextPx = pX;
                        nextPy = pY;
                    }
                    else
                    {
                        pX = nextPx; pY = nextPy;
                    }
                }
                else
                {
                    nextPx = pX;
                    nextPy = pY;
                }
            }
        }
    }
}
