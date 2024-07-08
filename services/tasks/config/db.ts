import mongoose from 'mongoose';

const connectDB = async () => {
    try{
        await mongoose.connect('link')
        console.log('Connection to MongoDB successfull');
    }
    catch(err){
        console.log('Connection to MongoDB failed:', err);
        process.exit(1);
    }
}

export default connectDB;