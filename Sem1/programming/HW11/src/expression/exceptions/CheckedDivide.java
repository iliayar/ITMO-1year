package expression.exceptions;

import expression.CommonExpression;
import expression.Divide;
import expression.DivisonByZeroException;
import expression.IntegerOverflowException;

public class CheckedDivide extends Divide {
    public CheckedDivide(CommonExpression a, CommonExpression b) {
        super(a, b);
    }


    @Override
    public int eval(int a, int b) throws DivisonByZeroException, IntegerOverflowException {
        if(b == 0) {
            throw new DivisonByZeroException(a,b);
        }
        if((long)a/(long)b > Integer.MAX_VALUE ||
                (long)a/(long)b < Integer.MIN_VALUE) {
            throw new IntegerOverflowException(a+"+"+b);
        }
        return a/b;
    }
}
