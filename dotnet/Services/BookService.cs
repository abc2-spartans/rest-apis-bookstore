public interface IBookService
{
    IEnumerable<Book> GetAllBooks();
    Book GetBookById(long id);
    Book AddBook(Book book);
    Book UpdateBook(long id, Book book);
    void DeleteBook(long id);
}

// filepath: Services/BookService.cs
public class BookService : IBookService
{
    private readonly BookstoreContext _context;

    public BookService(BookstoreContext context)
    {
        _context = context;
    }

    public IEnumerable<Book> GetAllBooks()
    {
        return _context.Books.ToList();
    }

    public Book GetBookById(long id)
    {
        var book = _context.Books.Find(id);
        if (book == null)
            throw new KeyNotFoundException($"Book not found with id: {id}");
        return book;
    }

    public Book AddBook(Book book)
    {
        _context.Books.Add(book);
        _context.SaveChanges();
        return book;
    }

    public Book UpdateBook(long id, Book bookDetails)
    {
        var book = GetBookById(id);
        book.Title = bookDetails.Title;
        book.Author = bookDetails.Author;
        book.Price = bookDetails.Price;

        _context.Books.Update(book);
        _context.SaveChanges();
        return book;
    }

    public void DeleteBook(long id)
    {
        var book = GetBookById(id);
        _context.Books.Remove(book);
        _context.SaveChanges();
    }
}