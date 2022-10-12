#[derive(PartialEq, Clone, Copy)]
pub enum Side{
    TOP, BOTTOM, WEST, EAST, SOUTH, NORTH
}

pub struct BlockFace{
    x:i64,
    y:i64,
    z:i64,
    side: Side,
    rotation: i32
}


fn get_coordinate_random(x:i128, y:i128, z:i128) -> i128 {
    let mut i:i128 = (x * 3129871) as i128 ^ z * 116129781 ^ y;
    i = i * i * 42317861 + i * 11;
    // println!("Random Coordinate {}", i);
    return i;
}

fn get_texture_type(x:i128, y:i128, z:i128) -> i128 {
    let texture_type =  ((get_coordinate_random(x, y, z) >> 16)%16).abs();
    // println!("Texture Type: {}", textureType);
    return texture_type;
}

fn compatible_rotation(generated_type:i128, bface: &BlockFace) -> bool {
    if generated_type == 0
    {
        return bface.rotation==3;
    }
    if generated_type == 1
    {
        return (bface.rotation==3 && (bface.side==Side::TOP||bface.side==Side::SOUTH))||
            (bface.rotation==2 && (bface.side==Side::WEST))||
            (bface.rotation==1 && (bface.side==Side::BOTTOM||bface.side==Side::NORTH))||
            (bface.rotation==0 && (bface.side==Side::EAST));
    }
    if generated_type == 2
    {
        return (bface.rotation==3 && (bface.side==Side::TOP||bface.side==Side::BOTTOM))||
            (bface.rotation==1 && (bface.side!=Side::TOP&&bface.side!=Side::BOTTOM));
    }
    if generated_type == 3
    {
        return (bface.rotation==3 && (bface.side==Side::BOTTOM||bface.side==Side::SOUTH))||
            (bface.rotation==2 && (bface.side==Side::EAST))||
            (bface.rotation==1 && (bface.side==Side::TOP||bface.side==Side::NORTH))||
            (bface.rotation==0 && (bface.side==Side::WEST));
    }
    if generated_type == 4
    {
        return (bface.rotation==3 && (bface.side!=Side::TOP&&bface.side!=Side::BOTTOM))||
            (bface.rotation==2 && (bface.side==Side::BOTTOM))||
            (bface.rotation==0 && (bface.side==Side::TOP));
    }
    if generated_type == 5
    {
        return (bface.rotation==3 && (bface.side==Side::WEST))||
            (bface.rotation==2 && (bface.side==Side::NORTH))||
            (bface.rotation==1 && (bface.side==Side::EAST))||
            (bface.rotation==0 && (bface.side==Side::TOP||bface.side==Side::BOTTOM||bface.side==Side::SOUTH));
    }
    if generated_type == 6
    {
        return (bface.rotation==1 && (bface.side!=Side::TOP&&bface.side!=Side::BOTTOM))||
            (bface.rotation==2 && (bface.side==Side::BOTTOM))||
            (bface.rotation==0 && (bface.side==Side::TOP));
    }
    if generated_type == 7
    {
        return (bface.rotation==3 && (bface.side==Side::WEST))||
            (bface.rotation==2 && (bface.side==Side::SOUTH||bface.side==Side::TOP||bface.side==Side::BOTTOM))||
            (bface.rotation==1 && (bface.side==Side::EAST))||
            (bface.rotation==0 && (bface.side==Side::NORTH));
    }
    if generated_type == 8
    {
        return (bface.rotation==1 && (bface.side==Side::TOP||bface.side==Side::BOTTOM))||
            (bface.rotation==3 && (bface.side!=Side::TOP&&bface.side!=Side::BOTTOM));
    }
    if generated_type == 9
    {
        return (bface.rotation==3 && (bface.side==Side::BOTTOM||bface.side==Side::NORTH))||
            (bface.rotation==2 && (bface.side==Side::EAST))||
            (bface.rotation==1 && (bface.side==Side::TOP||bface.side==Side::SOUTH))||
            (bface.rotation==0 && (bface.side==Side::WEST));
    }
    if generated_type == 10
    {
        return bface.rotation==1;
    }
    if generated_type == 11
    {
        return (bface.rotation==3 && (bface.side==Side::TOP||bface.side==Side::NORTH))||
            (bface.rotation==2 && (bface.side==Side::WEST))||
            (bface.rotation==1 && (bface.side==Side::BOTTOM||bface.side==Side::SOUTH))||
            (bface.rotation==0 && (bface.side==Side::EAST));
    }
    if generated_type == 12
    {
        return (bface.rotation==3 && (bface.side!=Side::TOP&&bface.side!=Side::BOTTOM))||
            (bface.rotation==2 && (bface.side==Side::TOP))||
            (bface.rotation==0 && (bface.side==Side::BOTTOM));
    }
    if generated_type == 13
    {
        return (bface.rotation==3 && (bface.side==Side::EAST))||
            (bface.rotation==2 && (bface.side==Side::TOP||bface.side==Side::BOTTOM||bface.side==Side::SOUTH))||
            (bface.rotation==1 && (bface.side==Side::WEST))||
            (bface.rotation==0 && (bface.side==Side::NORTH));
    }
    if generated_type == 14
    {
        return (bface.rotation==1 && (bface.side!=Side::TOP&&bface.side!=Side::BOTTOM))||
            (bface.rotation==2 && (bface.side==Side::TOP))||
            (bface.rotation==0 && (bface.side==Side::BOTTOM));
    }
    if generated_type == 15
    {
        return (bface.rotation==3 && (bface.side==Side::EAST))||
            (bface.rotation==2 && (bface.side==Side::NORTH))||
            (bface.rotation==1 && (bface.side==Side::WEST))||
            (bface.rotation==0 && (bface.side==Side::SOUTH||bface.side==Side::TOP||bface.side==Side::BOTTOM));
    }
    return false;
}

pub fn rotate90deg(input: Option<&Vec<BlockFace>>) -> Vec<BlockFace> {

    let mut result:Vec<BlockFace> = vec![];
    let formation = input.unwrap();
    for b in formation {
        
        let mut newside = Side::NORTH;
        let mut rotation = -1;
        
        if b.side == Side::TOP
        {
            newside = Side::TOP;
        }
        if b.side == Side::BOTTOM {newside = Side::BOTTOM;}
        if b.side == Side::WEST {newside = Side::SOUTH;}
        if b.side == Side::EAST {newside = Side::NORTH;}
        if b.side == Side::SOUTH {newside = Side::EAST;}
        if b.side == Side::NORTH {newside = Side::WEST;}

        if b.side == Side::TOP
        {
            rotation = (b.rotation+3)%4;
        }
        else if b.side == Side::BOTTOM
        {
            rotation = (b.rotation+1)%4;
        }
        else
        {
            rotation = b.rotation;
        }
        result.push(BlockFace { x: b.z, y: b.y, z: b.z, side: newside, rotation });
    }
    return result;
    
    
}





fn main() {


    let mut formation: Vec<BlockFace> = Vec::new();
    let mut rotations: Vec<Vec<BlockFace>> = Vec::new();

    //START HERE

    //INPUT YOUR BLOCKFACES HERE
    //x y and z are relative coords, not actual (thats what you are looking for)
    //rotation is found with the "L" on the netherack (on the readme)
    //L rotation
    //top left  = 0
    //top right  = 1
    //bottom right = 2
    //bottom left  = 3

    //Example Formation
    formation.push(BlockFace { x: 0, y: 0, z: 0, side: Side::NORTH, rotation: 0 });
    formation.push(BlockFace { x: 0, y: 1, z: 0, side: Side::NORTH, rotation: 0 });
    formation.push(BlockFace { x: 0, y: 2, z: 0, side: Side::NORTH, rotation: 0 });
    formation.push(BlockFace { x: 0, y: 3, z: 0, side: Side::NORTH, rotation: 0 });
    formation.push(BlockFace { x: 0, y: 4, z: 0, side: Side::NORTH, rotation: 0 });
    formation.push(BlockFace { x: 0, y: 5, z: 0, side: Side::NORTH, rotation: 0 });
    formation.push(BlockFace { x: 0, y: 6, z: 0, side: Side::NORTH, rotation: 0 });
    formation.push(BlockFace { x: 0, y: 7, z: 0, side: Side::NORTH, rotation: 0 });
    formation.push(BlockFace { x: 0, y: 8, z: 0, side: Side::NORTH, rotation: 0 });
    formation.push(BlockFace { x: 0, y: 9, z: 0, side: Side::NORTH, rotation: 0 });
    formation.push(BlockFace { x: 0, y: 10, z: 0, side: Side::NORTH, rotation: 0 });
    formation.push(BlockFace { x: 1, y: 10, z: 0, side: Side::SOUTH, rotation: 0 });
    formation.push(BlockFace { x: 1, y: 10, z: 0, side: Side::SOUTH, rotation: 1 });
    formation.push(BlockFace { x: 2, y: 10, z: 0, side: Side::SOUTH, rotation: 0 });
    formation.push(BlockFace { x: 7, y: 10, z: 0, side: Side::SOUTH, rotation: 0 });


    //Search Range parameters
    //Make sure Y min and Y max are at least one apart. It will do whatever the lower one i
    let xmin:i64 = -2000;
    let xmax:i64 = 2000;
    let zmin:i64 = -2000;
    let zmax:i64 = 2000;
    let ymin:i64 = 59;
    let ymax:i64 = 60;


    let use_all_rotations = false;//set this to true if you don't know which direction is north

    rotations.push(formation);


    if use_all_rotations
    {
        for _n in 0..3 {
            rotations.push(rotate90deg(rotations.get(rotations.len() - 1)));
        }
    }



    for x in xmin..xmax {
        for z in zmin..zmax {
            for y in ymin..ymax {
                for f in &rotations{
                    let mut found = true;
                    for b in f{
                        let texture = get_texture_type((x + b.x) as i128, (y + b.y) as i128, (z + b.z) as i128);
                        if !compatible_rotation(texture, b)
                        {
                            found=false;
                            break;
                        }
                    }
                    if found {
                        println!("Found Possible Coordinates at : {}, {}, {}", x,y,z);
                        return;
                    }
                }
            }
        }
    }

    println!("Finished!");

}




