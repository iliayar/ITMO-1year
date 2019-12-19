package expression.parser;

import expression.*;

public class ExpressionParser extends BaseParser {

    public ExpressionParser() {
    }

    public ExpressionParser(ExpressionSource src) {
        this.src = src;
        nextChar();
    }


    @Override
    public CommonExpression parse(String src) {
        this.src = new StringSource(src);
        nextChar();
        return parseExpression();
    }

    private Const parseConst(boolean isMinus) {
        StringBuilder sb = new StringBuilder();
        if(isMinus) {
            sb.append('-');
        }
        while(between('0','9')) {
            sb.append(ch);
            nextChar();
        }
        try {
            return new Const(Integer.parseInt(sb.toString()));
        } catch (NumberFormatException e) {
            throw error("Integer expected " + ch + " found");
        }
    }

    private char parseOperation() {
        if(test('+')) {
            return '+';
        } else if(test('-')) {
            return '-';
        } else if(test('*')) {
            return  '*';
        } else if(test('/')) {
            return '/';
        } else if(test('<')) {
            test('<');
            return '<';
        } else if(test('>')) {
            test('>');
            return '>';
        }
        throw error("One of \'+, -, *, /\' expected");
    }

    private CommonExpression parseOperand() {
        if(between('0','9')) {
            return parseConst(false);
        } else if(test('x')) {
            return new Variable("x");
        } else if(test('y')) {
            return new Variable("y");
        } else if(test('z')) {
            return new Variable("z");
        } else if(test('-')) {
            skipWhitespace();
            if(between('0','9')) {
                return parseConst(true);
            } else if(test('(')) {
                CommonExpression expr = parseExpression();
                test(')');
                return new Inverse(expr);
            } else {
                return new Inverse(parseOperand());
            }
        } else if(test('d')) {
            expect("igits");
            skipWhitespace();
            return new Digits(parseExpression());
        } else if(test('r')) {
            expect("everse");
            skipWhitespace();
            return new Reverse(parseExpression());
        }

        throw error("Operand expected " + ch + " found");
    }

    public CommonExpression parseExpression() {
        return parse3PriorExpression();
    }

    public CommonExpression parse3PriorExpression() {
        skipWhitespace();

        CommonExpression firstOperand = parse2PriorExpression();

        skipWhitespace();
        if( test('\0')) {
            return firstOperand;
        }
        while(in("<>")) {
            CommonExpression secondOperand = null;

            char operation = parseOperation();
            skipWhitespace();
            secondOperand = parse2PriorExpression();
            skipWhitespace();
            if(operation == '<') {
                firstOperand = new ShiftLeft(firstOperand, secondOperand);
            } else {
                firstOperand = new ShiftRight(firstOperand, secondOperand);
            }

        }
        return firstOperand;
    }

    public CommonExpression parse2PriorExpression() {

        skipWhitespace();

        CommonExpression firstOperand = parse1PriorExpression();

        skipWhitespace();
        if( test('\0')) {
            return firstOperand;
        }
        while(in("+-")) {
            CommonExpression secondOperand = null;

            char operation = parseOperation();
            skipWhitespace();
            secondOperand = parse1PriorExpression();
            skipWhitespace();
            if(operation == '+') {
                firstOperand = new Add(firstOperand, secondOperand);
            } else {
                firstOperand = new Subtract(firstOperand, secondOperand);
            }

        }
        return firstOperand;
    }

    private CommonExpression parse1PriorExpression() {

        skipWhitespace();

        CommonExpression firstOperand = null;

        if(test('(')) {
            firstOperand = parseExpression();
            test(')');
        } else {
            firstOperand = parseOperand();
        }
        skipWhitespace();
        if(test('\0')) {
            return firstOperand;
        }
        while(in("*/")) {
            CommonExpression secondOperand = null;

            char operation = parseOperation();
            skipWhitespace();
            if(test('(')) {
                secondOperand = parseExpression();
                test(')');
            } else {
                secondOperand = parseOperand();
            }
            skipWhitespace();
            if(operation == '*') {
                firstOperand = new Multiply(firstOperand, secondOperand);
            } else {
                firstOperand = new Divide(firstOperand, secondOperand);
            }

        }
        return firstOperand;
    }

    private void skipWhitespace() {
        while (test(' ') || test('\r') || test('\n') || test('\t')) {
            // skip
        }
    }

}
