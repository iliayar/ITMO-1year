package expression.exceptions;

import expression.exceptions.ExpressionParser;

public class Main {

    public static void main(String[] args) {

        System.out.println(new ExpressionParser().parse("-2147483649").toString());

    }

}