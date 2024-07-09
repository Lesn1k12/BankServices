import jwt from 'jsonwebtoken';

export const isValidTitle = (title: string) => {
    if (title.length < 3 || title.length > 30) {
        return false;
    }
    return true;
}

export const isValidDescription = (description: string) => {
    if (description.length < 3 || description.length > 100) {
        return false;
    }
    return true;
}

export const isValidToken = (token: string): boolean => {

    if (!token || !process.env.TOKEN_SECRET) {
        return false
    }

    try {
        jwt.verify(token, process.env.TOKEN_SECRET as string);
        return true;
    } catch (error) {
        console.error("Token verification failed:", error);
        return false;
    }
};