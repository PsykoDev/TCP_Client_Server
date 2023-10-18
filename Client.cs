using System;
using System.IO;
using System.Net.Sockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

class Client
{
    static async Task Main()
    {
        Console.WriteLine("Bienvenue sur le client de chat multicanal!");
        Console.Write("Entrez votre nom d'utilisateur : ");
        string username = Console.ReadLine();

        TcpClient client = new TcpClient("127.0.0.1", 1234);
        NetworkStream stream = client.GetStream();

        byte[] pseudo = Encoding.UTF8.GetBytes(username);
        await stream.WriteAsync(pseudo, 0, pseudo.Length);

        var cancellationTokenSource = new CancellationTokenSource();

        var receiveThread = new Thread(async () =>
        {
            var buff = new char[4096];
            var reader = new StreamReader(stream);
            while (true)
            {
                try
                {
                    int bytesRead = await reader.ReadAsync(buff, 0, buff.Length);
                    if (bytesRead == 0)
                    {
                        break;
                    }
                    Console.WriteLine(new string(buff, 0, bytesRead));
                }
                catch (Exception e)
                {
                    Console.WriteLine(e);
                    break;
                }
            }
        });

        receiveThread.Start();

        while (true)
        {
            string message = Console.ReadLine();
            if (string.IsNullOrEmpty(message))
            {
                cancellationTokenSource.Cancel();
                break;
            }

            byte[] data = Encoding.UTF8.GetBytes($"{message}\n");
            await stream.WriteAsync(data, 0, data.Length);
        }

        client.Close();
    }
}
