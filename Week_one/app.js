const age= window.prompt("input your age")

const ageInt = parseInt(age);
if(ageINt==18)
{
    console.log("are you sure?")
}

else if (ageInt<18)
{
    console.log("you are underage")
}
else if (ageInt>18)
{
    console.log("you are over age")
}

