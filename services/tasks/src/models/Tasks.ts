import { Schema, model } from 'mongoose';

interface Task {
    user: Schema.Types.ObjectId;
    title: string;
    description: string;
}

const TaskSchema = new Schema<Task>({
    title: { type: String, required: true },
    description: { type: String, required: true },
});

const Task = model<Task>('Task', TaskSchema);

export default Task;