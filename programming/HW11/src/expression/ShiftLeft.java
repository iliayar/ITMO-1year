package expression;

public class ShiftLeft extends Operation {


    public ShiftLeft(Expression a, Expression b) {
        super(a, b);
    }

    @Override
    protected String getSymbol() {
        return "<<";
    }

    @Override
    protected int eval(int a, int b) {
        return a << b;
    }

    @Override
    protected long eval(long a, long b) {
        return (a << b);
    }


    @Override
    public int getPrior() {
        return 3;
    }

    @Override
    public boolean isCommutative() {
        return false;
    }
}
