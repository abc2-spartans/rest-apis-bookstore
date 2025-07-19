import {
  Controller,
  Get,
  Post,
  Put,
  Delete,
  Param,
  Body,
  NotFoundException,
} from '@nestjs/common';
import { BooksService } from './books.service';
import { Book } from './book.entity';

@Controller()
export class BooksController {
  constructor(private readonly booksService: BooksService) {}

  @Get(['/', '/health'])
  health() {
    return {
      status: 'healthy',
      timestamp: new Date().toISOString(),
      service: 'Bookstore API',
      apiURL: 'http://localhost:4200/api/v1/books',
    };
  }

  @Get('/api/v1/books')
  findAll(): Promise<Book[]> {
    return this.booksService.findAll();
  }

  @Get('/api/v1/books/:id')
  async findOne(@Param('id') id: string): Promise<Book> {
    const book = await this.booksService.findOne(Number(id));
    if (!book) throw new NotFoundException('Book not found');
    return book;
  }

  @Post('/api/v1/books')
  create(@Body() book: Partial<Book>): Promise<Book> {
    return this.booksService.create(book);
  }

  @Put('/api/v1/books/:id')
  async update(
    @Param('id') id: string,
    @Body() book: Partial<Book>,
  ): Promise<Book> {
    const updated = await this.booksService.update(Number(id), book);
    if (!updated) throw new NotFoundException('Book not found');
    return updated;
  }

  @Delete('/api/v1/books/:id')
  async remove(@Param('id') id: string): Promise<void> {
    const book = await this.booksService.findOne(Number(id));
    if (!book) throw new NotFoundException('Book not found');
    await this.booksService.remove(Number(id));
  }
}
