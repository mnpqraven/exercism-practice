using System;

static class LogLine
{
    public static string Message(string logLine)
    {
      int starts = logLine.IndexOf(" ");
      string res = logLine.Substring(starts);
      return res.Trim();
    }

    public static string LogLevel(string logLine)
    {
      int ends = logLine.IndexOf("]") -1;
      string res = logLine.Substring(1,ends);
      return res.ToLower();
    }

    public static string Reformat(string logLine)
    {
      string info = logLine.Substring(logLine.IndexOf(" "));
      string type = logLine.Substring(1, logLine.IndexOf("]")-1);
      return $"{info.Trim()} ({type.ToLower()})";
    }
}
