import express from 'express';
import connectDB from './config/db';
import router from './routes/taskRoutes';
const app = express()
const port = 8084

connectDB();

app.use(express.json());
app.use('/tasks', router);

app.listen(port, () => console.log(`Server running on port ${port}`));


