using System.Collections;
using System.Numerics;
using System.Text;

using static Log;

static class Tasks
{
    const string tasks = """
    Tasks:
        1. Count negative elements in the matrix.
        2. Swap the corresponding elements of the first (technically 0) row and the main diagonal.
        3. Sort the side diagonal of the matrix from the minimum right-top to the maximum left-bottom.
        4. Sort the columns of the matrix by non-decreasing minimum element.
    Or type 'exit' to exit the program.
    """;

    public static void Main()
    {
        var running = true;

        while (running)
        {
            Message(tasks);

            var value = Request.Value<string>("Select task");

            try
            {
                if (ParseTask(value) is Action task)
                    task();
                else
                    break;
            }
            catch (FormatException e)
            {
                Error(e.Message);
            }
        }
    }

    static Action? ParseTask(string s)
    {
        var input = s.Trim().ToLower();

        if (input == "exit")
        {
            return null;
        }

        var isParsed = int.TryParse(input, null, out int parsed);

        if (!isParsed)
        {
            throw new FormatException($"Invalid input");
        }

        return parsed switch
        {
            1 => Task1,
            2 => Task2,
            3 => Task3,
            4 => Task4,
            var x => throw new FormatException($"Unknown task: {x}"),
        };
    }

    // Count negative elements in the matrix.
    static void Task1()
    {
        var matrix = Request.Matrix<int>();
        var count = 0;

        foreach (var item in matrix)
        {
            if (item < 0) ++count;
        }

        Print(count);
    }

    // Swap the corresponding elements of the first row and the main diagonal.
    //
    // Assume that the matrix is guaranteed to be square.
    static void Task2()
    {
        var matrix = Request.SquareMatrix<int>();

        for (int i = 0; i < matrix.Rows; ++i)
        {
            matrix.SwapElements((0, i), (i, i));
        }

        Print(matrix);
    }

    // Sort the side diagonal of the matrix from the minimum right-top to the
    // maximum left-bottom.
    //
    // Assume that the matrix is guaranteed to be square.
    static void Task3()
    {
        var matrix = Request.SquareMatrix<int>();
        var max = matrix.Rows - 1;

        for (int i = 0; i <= max; ++i)
        {
            var swapped = false;

            for (int j = i + 1; j <= max; ++j)
            {
                if (matrix[i, max - 1] > matrix[j, max - j])
                {
                    matrix.SwapElements((i, max - i), (j, max - j));
                    swapped = true;
                }
            }

            if (!swapped) break;
        }

        Print(matrix);
    }

    // Sort the columns of the matrix by non-decreasing minimum element.
    static void Task4()
    {
        var matrix = Request.Matrix<int>();

        // WARNING: array copies tsunami
        matrix.Transpose();

        var cols = matrix
            .ToMultiArray()
            .OrderBy(col => col.Max())
            .ToArray();

        matrix = new Matrix<int>(cols);
        matrix.Transpose();

        Print(matrix);
    }
}

static class Log
{
    public static void Message(string message = "")
    {
        Console.Error.WriteLine("\x1B[35;1m" + message + "\x1B[0m");
    }

    public static void Error(string message = "")
    {
        Console.Error.WriteLine("\x1B[31;1mError!\x1B[0m \x1B[1m" + message + "\x1B[0m");
    }

    public static void Debug(string message = "")
    {
        Console.Error.WriteLine("\x1B[33m" + message + "\x1B[0m");
    }

    public static void Print(object message)
    {
        Console.WriteLine(message);
    }
}

class Matrix<T> : IEnumerable<T> where T : INumber<T>
{
    public T[,] Items { get; private set; } = new T[0, 0];

    public T this[int row, int col]
    {
        get => Items[row, col];
        set => Items[row, col] = value;
    }

    public int Rows => Items.GetLength(0);
    public int Cols => Items.GetLength(1);

    public Matrix(int rows, int cols)
    {
        Items = new T[rows, cols];
    }

    public Matrix(int size)
    {
        Items = new T[size, size];
    }

    public Matrix(T[,] items)
    {
        Items = items;
    }

    public Matrix(T[][] items)
    {
        if (items.Length == 0) return;

        var rows = items.Length;
        var cols = items[0].Length;

        Items = new T[rows, cols];

        for (int row = 0; row < rows; row++)
        {
            if (items[row].Length != cols)
                throw new ArgumentException(
                    $"Unexpected sub-array size {items[row].Length}, expected {cols}",
                    nameof(items));

            for (int col = 0; col < cols; col++)
            {
                Items[row, col] = items[row][col];
            }
        }
    }

    public T[][] ToMultiArray()
    {
        var items = new T[Rows][];

        for (int row = 0; row < Rows; ++row)
        {
            items[row] = new T[Cols];
        }

        for (int row = 0; row < Rows; row++)
        {
            for (int col = 0; col < Cols; col++)
            {
                items[row][col] = Items[row, col];
            }
        }

        return items;
    }

    public void SwapElements((int x, int y) a, (int x, int y) b)
    {
        (this[a.x, a.y], this[b.x, b.y]) = (this[b.x, b.y], this[a.x, a.y]);
    }

    public override string ToString()
    {
        var buffers = new StringBuilder[Rows];

        for (int i = 0; i < buffers.Length; ++i)
        {
            buffers[i] = new StringBuilder();
        }

        for (int row = 0; row < Rows; ++row)
        {
            var buffer = buffers[row];

            for (int col = 0; col < Cols; ++col)
            {
                if (col > 0) buffer.Append(' ');
                buffer.Append(this[row, col]);
            }
        }

        return string.Join('\n', buffers.AsEnumerable());
    }

    public void Transpose()
    {
        if (Rows == Cols)
        {
            for (int row = 0; row < Rows; ++row)
            {
                for (int col = row + 1; col < Cols; col++)
                {
                    if (row != col) SwapElements((row, col), (col, row));
                }
            }
        }
        else
        {
            var items = new T[Cols, Rows];

            for (int row = 0; row < Rows; ++row)
            {
                for (int col = 0; col < Cols; col++)
                {
                    items[col, row] = this[row, col];
                }
            }

            Items = items;
        }
    }

    public IEnumerator<T> Diagonal(bool primary = true)
    {
        var max = Rows - 1;

        for (int i = 0; i <= max; ++i)
        {
            var col = primary ? i : (max - i);
            yield return this[i, col];
        }
    }

    public IEnumerator<T> GetEnumerator()
    {
        foreach (T item in Items)
        {
            yield return item;
        }
    }

    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
}


static class Request
{
    public static T Value<T>(Func<string, T> parse, string prompt)
    {
        while (true)
        {
            Console.Error.Write($"\x1b[36;1m{prompt}:\x1b[0m ");

            var input = Console.ReadLine() ?? "";
            try
            {
                return parse(input);
            }
            catch (Exception e) when (e is FormatException or OverflowException)
            {
                Error($"Can't parse the input: {e.Message}");
            }
        }
    }

    public static T Value<T>(string prompt)
        where T : IParsable<T>
    {
        return Request.Value(input => T.Parse(input, null), prompt);
    }

    public static T[] Seq<T>()
        where T : IParsable<T>
    {
        while (true)
        {
            var success = true;
            var inputs = (Console.ReadLine() ?? "").Split();
            var items = new T[inputs.Length];
            var i = 0;

            foreach (var input in inputs)
            {
                try
                {
                    items[i] = T.Parse(input.Trim(), null);
                    ++i;
                }
                catch (Exception e) when (e is FormatException or OverflowException)
                {
                    Error($"Can't parse the input: {e.Message}");
                    success = false;
                    break;
                }
            }

            if (success) return items;
        }
    }

    public static T[] FixedSeq<T>(int size)
        where T : IParsable<T>
    {
        while (true)
        {
            var array = Request.Seq<T>();

            if (array.Length == size)
            {
                return array;
            }

            Error($"Unexpected input array size: {array.Length}, expected {size}");
        }
    }

    public static Matrix<T> Matrix<T>()
        where T : INumber<T>
    {
        var rows = Request.Value(NaturalInt, "Input matrix rows count");
        Matrix<T>? matrix = null;

        for (int i = 0; i < rows; ++i)
        {
            if (matrix is null)
            {
                var row = Request.Seq<T>();
                matrix = new Matrix<T>(rows, row.Length);

                for (int j = 0; j < row.Length; ++j)
                {
                    matrix[0, j] = row[j];
                }
            }
            else
            {
                var row = Request.FixedSeq<T>(matrix.Cols);

                for (int j = 0; j < matrix.Cols; ++j)
                {
                    matrix[i, j] = row[j];
                }
            }
        }

        Message($"Typed matrix:\n{matrix}");
        return matrix!;
    }

    public static Matrix<T> SquareMatrix<T>()
        where T : INumber<T>
    {
        var size = Request.Value(NaturalInt, "Input square matrix size");
        var matrix = new Matrix<T>(size);

        for (int i = 0; i < size; ++i)
        {
            var row = Request.FixedSeq<T>(size);

            for (int j = 0; j < size; ++j)
            {
                matrix[i, j] = row[j];
            }
        }

        Message($"Typed matrix:\n{matrix}");
        return matrix;
    }

    private static int NaturalInt(string input)
    {
        var value = int.Parse(input);

        if (value <= 0)
        {
            throw new FormatException("Unexpected negative integer");
        }

        return value;
    }
}
