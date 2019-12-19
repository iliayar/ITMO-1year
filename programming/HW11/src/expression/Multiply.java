package expression;

import java.math.BigInteger;

public class Multiply extends Operation {
    public Multiply(CommonExpression a, CommonExpression b) {
        super(a, b);
    }

    @Override
    protected String getSymbol() {
        return "*";
    }

    @Override
    protected int eval(int a, int b) {
        return a*b;
    }

    @Override
    protected BigInteger eval(BigInteger a, BigInteger b) {
        return a.multiply(b);
    }


    @Override
    public int getPrior() {
        return 1;
    }
    @Override
    public boolean isCommutative() {
        return true;
    }
}
