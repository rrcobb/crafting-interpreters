package jlox;

import java.util.List;

// prints a reverse polish notation string representation of AST nodes
class RpnPrinter implements Expr.Visitor<String>, Stmt.Visitor<Void> {
  public static void main(String[] args) {
    String source = "1 + ((0+1, 6*7) ? 2 : 3 + 8) + 4";
    Scanner scanner = new Scanner(source);
    List<Token> tokens = scanner.scanTokens();
    Parser parser = new Parser(tokens);
    List<Stmt> statements = parser.parse();

    new RpnPrinter().print(statements);
  }

  String print(Expr expr) {
    return expr.accept(this);
  }

  Void print(List<Stmt> statements) {
    for (Stmt statement : statements) {
      statement.accept(this);
    }
  }

  @Override
  public Void visitPrintStmt(Stmt.Print stmt) {
    String value = print(stmt.expression);
    System.out.println(value);
    return null;
  }


  @Override
  public String visitBinaryExpr(Expr.Binary expr) {
    // binary expression goes left right op
    return expr.left.accept(this) + " " 
      + expr.right.accept(this) + " "
      + expr.operator.lexeme; 
  }

  @Override
  public String visitTernaryExpr(Expr.Ternary expr) {
    // ternary 
    // https://stackoverflow.com/a/16930865/3574917
    // first second third : ?
    return expr.first.accept(this) + " "
      + expr.second.accept(this) + " "
      + expr.third.accept(this) + " "
      +  ": ? ";
  }

  @Override
  public String visitGroupingExpr(Expr.Grouping expr) {
    // I guess just... print the expr?
    return expr.expression.accept(this);
  }

  @Override
  public String visitLiteralExpr(Expr.Literal expr) {
    // print literal, just like AST printer
    if (expr.value == null) return "nil";
    return expr.value.toString();
  }

  @Override
  public String visitUnaryExpr(Expr.Unary expr) {
    // not sure how unary expressions go...
    // maybe just expr op?
    // note: this is wrong, see https://github.com/munificent/craftinginterpreters/blob/master/note/answers/chapter05_representing.md
    // have to use a different symbol for unary and binary negation to distinguish them
    // bob uses ~ for unary negation
    return expr.right.accept(this) + " " + expr.operator.lexeme;
  }

}
