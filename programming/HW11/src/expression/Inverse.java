package expression;

import java.math.BigInteger;

public class Inverse extends UnaryOperation {
    public Inverse(Expression a) {
        super(a);
    }

    @Override
    protected String getSymbol() {
        return "-";
    }

    @Override
    protected int eval(int a) {
        return -a;
    }

    @Override
    protected BigInteger eval(BigInteger a) {
        return a.multiply(BigInteger.valueOf(-1));
    }


    @Override
    public int getPrior() {
        return 0;
    }
}
