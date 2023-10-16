using System;
using System.IO;
using System.Net.Sockets;

class Client
{
    static void Main()
    {
        Console.WriteLine("Bienvenue sur le client de chat multicanal!");
        Console.Write("Entrez votre nom d'utilisateur : ");
        string username = Console.ReadLine();

        TcpClient client = new TcpClient("127.0.0.1", 1234);
        NetworkStream stream = client.GetStream();

        // Crée un thread pour lire les messages du serveur
        var receiveThread = new System.Threading.Thread(() =>
        {
            StreamReader reader = new StreamReader(stream);
            while (true)
            {
                try
                {
                    string message = reader.ReadLine();
                    Console.WriteLine(message);
                }
                catch (Exception e)
                {
                    Console.WriteLine(e);
                    throw;
                }

            }
        });
        
        receiveThread.Start();

        while (true)
        {
            string message = Console.ReadLine();
            byte[] data = System.Text.Encoding.ASCII.GetBytes($"{username}: {message}");
            stream.Write(data, 0, data.Length);
        }
        

    }
}
