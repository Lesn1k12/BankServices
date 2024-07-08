import express, {Express, Request, Response } from 'express';
import connectDB from '../config/db';
const app = express()
const port = 8084

connectDB();

app.use(express.json());
app.use('/api/tasks', require('./routes/taskRoutes'));

app.listen(port, () => console.log(`Server running on port ${port}`));


