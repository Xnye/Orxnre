namespace Orxnre;

public abstract class BlockInfo
{
	public class SingleBlockInfo
	{
		public string Title = "　";
		public ConsoleColor Color = ConsoleColor.White;
		public string Explain = "";
	}

	public static readonly SingleBlockInfo[] List = {
		new() { Title = "　", Color = ConsoleColor.White, Explain = "" },
		new() { Title = "土", Color = ConsoleColor.DarkYellow, Explain = "地上只有平平无奇的干土" },
		new() { Title = "草", Color = ConsoleColor.DarkGreen, Explain = "高大的杂草与荆棘让行走变得困难" },
		new() { Title = "石", Color = ConsoleColor.DarkGray, Explain = "杂乱无章的石块组成了地面" }
	};
}