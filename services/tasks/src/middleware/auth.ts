import { Request, Response, NextFunction } from 'express';
import jwt, { JwtPayload } from 'jsonwebtoken';

export interface CustomRequest extends Request {
    user?: { id: string };
}

export const validateToken = (req: CustomRequest, res: Response, next: NextFunction) => {
    const token = req.headers.authorization?.split(' ')[1] ?? '';
    if (!token) {
        return res.status(401).send('Access Denied');
    }
    if(!process.env.TOKEN_SECRET) {
        return res.status(500).send('Internal Server Error');
    }

    const secret = process.env.TOKEN_SECRET as string;

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