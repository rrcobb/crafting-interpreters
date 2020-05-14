package jlox;

// prints a reverse polish notation string representation of AST nodes
class RpnPrinter implements Expr.Visitor<String> {
  public static void main(String[] args) {
    Expr expression = new Expr.Binary(
        new Expr.Grouping(
          new Expr.Binary(
            new Expr.Literal(1),
            new Token(TokenType.PLUS, "+", null, 1),
            new Expr.Literal(2))),
        new Token(TokenType.STAR, "*", null, 1),
        new Expr.Grouping(
          new Expr.Binary(
            new Expr.Literal(4),
            new Token(TokenType.MINUS, "-", null, 1),
            new Expr.Literal(3)
            )));

    System.out.println(new RpnPrinter().print(expression));
  }

  String print(Expr expr) {
    return expr.accept(this);
  }

  @Override
  public String visitBinaryExpr(Expr.Binary expr) {
    // binary expression goes left right op
    return expr.left.accept(this) + " " 
      + expr.right.accept(this) + " "
      + expr.operator.lexeme; 
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
