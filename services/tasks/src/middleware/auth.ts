import { Request, Response, NextFunction } from 'express';
import jwt, { JwtPayload } from 'jsonwebtoken';
import { isValidToken } from '../utils/validator';

const TOKEN_SECRET = "nYSvA9hsWvSZT/AOMcmiNze/YGtkwEFUMfCbos0LTgM="

export interface CustomRequest extends Request {
    user?: { id: string };
}

export const validateToken = (req: CustomRequest, res: Response, next: NextFunction) => {
    const token = req.headers.authorization?.split(' ')[1] ?? '';

    if (!isValidToken(token)) {
        return res.status(401).send('Invalid Token');
    }
    
    const secret = TOKEN_SECRET as string;

    try{
        const decoded = jwt.verify(token, secret) as JwtPayload;

        if (!decoded) {
            return res.status(401).send('Invalid Token');
        }

        req.user = { id: decoded.id}
        next();
        
    } catch{
        return res.status(401).send('Invalid Token');
    }
}