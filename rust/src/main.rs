#![allow(warnings)]

use std::io::Write;
use std::ffi::CString;
use raylib::prelude::*;
use rand::Rng;

const Debug          : bool = false;
const Game_Width     : i32  = 700;
const Game_Height    : i32  = 700;
const Ball_Radius    : f32  = 22.0;
const Ball_Fill      : f32  = 15.0;
const Starting_Balls : i32  = 4;

#[derive(Copy, Clone)]
struct Ball_Type {
    X: i32,
    Y: i32
}

#[derive(Copy, Clone)]
enum Movement_State_Type {
    Standby,
    Moving
}

#[derive(Copy, Clone)]
struct Movement_Operation_Type {
    Ball          : u32,
    Mouse_X_Start : i32,
    Mouse_Y_Start : i32,
    Ball_X_Start  : i32,
    Ball_Y_Start  : i32
}

fn Random_X() -> i32 {
    return rand::thread_rng().gen_range(Ball_Radius * 2.0 .. Game_Width as f32 - Ball_Radius * 2.0) as i32;
}

fn Random_Y() -> i32 {
    return rand::thread_rng().gen_range(Ball_Radius * 2.0 .. Game_Height as f32 - Ball_Radius * 2.0) as i32;
}
    
fn Intersections_Present(Balls: Vec<Ball_Type>) -> bool {
   let mut Lines : usize = Balls.len();
   let mut A     : Ball_Type;
   let mut B     : Ball_Type;
   let mut C     : Ball_Type;
   let mut D     : Ball_Type;
   let mut D1    : f32;
   let mut D2    : f32;
   let mut D3    : f32;
   let mut D4    : f32;
        
   for First in 1 .. Lines - 1 + 1 {
       for Second in First + 1 .. Lines - 1 + 1{
           A = Balls[First - 1];
           B = Balls[First];
           C = Balls[Second - 1];
           D = Balls[Second];
       
           D1 = (A.X - C.X) as f32 * (B.Y - C.Y) as f32 -
                (A.Y - C.Y) as f32 * (B.X - C.X) as f32;
           D2 = (A.X - D.X) as f32 * (B.Y - D.Y) as f32 -
                (A.Y - D.Y) as f32 * (B.X - D.X) as f32;
           D3 = (C.X - A.X) as f32 * (D.Y - A.Y) as f32 -
                (C.Y - A.Y) as f32 * (D.X - A.X) as f32;
           D4 = (C.X - B.X) as f32 * (D.Y - B.Y) as f32 -
                (C.Y - B.Y) as f32 * (D.X - B.X) as f32;

           if D1 * D2 < 0.0 && D3 * D4 < 0.0 {
               return true
           }
       }
   }
   for First in 1 .. Lines - 1 + 1 {
       A = Balls[0];
       B = Balls[Lines - 1];
       C = Balls[First - 1];
       D = Balls[First];
       
       D1 = (A.X - C.X) as f32 * (B.Y - C.Y) as f32 -
            (A.Y - C.Y) as f32 * (B.X - C.X) as f32;
       D2 = (A.X - D.X) as f32 * (B.Y - D.Y) as f32 -
            (A.Y - D.Y) as f32 * (B.X - D.X) as f32;
       D3 = (C.X - A.X) as f32 * (D.Y - A.Y) as f32 -
            (C.Y - A.Y) as f32 * (D.X - A.X) as f32;
       D4 = (C.X - B.X) as f32 * (D.Y - B.Y) as f32 -
            (C.Y - B.Y) as f32 * (D.X - B.X) as f32;

       if D1 * D2 < 0.0 && D3 * D4 < 0.0 {
           // An intersection is present when the left side determinate
           // is opposite sign of right side of determinant for both
           // pairs D1, D2 and D3, D4.
           return true
       }
   }
   false
}

fn New_Level(Balls: &mut Vec<Ball_Type>, Level_Complete: &mut bool) {
    let mut Ball_Count = Balls.len();
    while true {
        Balls.clear();
        for Index in 1 .. Ball_Count + 2 
            // For loops do not iterate over each element in the range specified. So an additional
            // range unit is added to the end (+1).
            + 1 
        {
            let mut Ball: Ball_Type = Ball_Type {X: 0, Y: 0};
            Ball.X = Random_X();
            Ball.Y = Random_Y();
            Balls.push(Ball);
        }
        *Level_Complete = false;
        if Intersections_Present(Balls.clone()) {
            break;
        }
    }
}

fn Update_Game
    (Balls: &mut Vec<Ball_Type>, 
     Level_Complete: &mut bool, 
     rl: &RaylibHandle,
     Movement_State: &mut Movement_State_Type,
     Movement_Operation: &mut Movement_Operation_Type,
     Clear: &mut bool,
     Level_Complete_Sound: &mut raylib::ffi::Sound)
{

    if *Level_Complete && rl.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) {
        New_Level (Balls, Level_Complete);
    }

    let Balls_Length = Balls.len();

    match Movement_State {
        Movement_State_Type::Standby =>
            if rl.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) {
                for Index in 0 .. Balls_Length {
                    if ((rl.get_mouse_x() - (Balls[Index].X as i32)).abs() < Ball_Radius as i32) &&
                       ((rl.get_mouse_y() - (Balls[Index].Y as i32)).abs() < Ball_Radius as i32) {
                        Movement_Operation.Ball          = Index as u32;
                        Movement_Operation.Mouse_X_Start = rl.get_mouse_x();
                        Movement_Operation.Mouse_Y_Start = rl.get_mouse_y();
                        Movement_Operation.Ball_X_Start  = Balls [Index].X;
                        Movement_Operation.Ball_Y_Start  = Balls [Index].Y;
                        *Movement_State                  = Movement_State_Type::Moving;
                    }
                }
            } else {
                if Intersections_Present(Balls.clone()) {
                    *Clear = false;
                } else {
                    *Clear = true;
                }
            }
        Movement_State_Type::Moving =>
        {
            if rl.is_mouse_button_down(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) {
                Balls[Movement_Operation.Ball as usize].X = Movement_Operation.Ball_X_Start - (Movement_Operation.Mouse_X_Start - rl.get_mouse_x() as i32);
                Balls[Movement_Operation.Ball as usize].Y = Movement_Operation.Ball_Y_Start - (Movement_Operation.Mouse_Y_Start - rl.get_mouse_y() as i32);
            } else {
                *Movement_State = Movement_State_Type::Standby;
                if !Intersections_Present(Balls.clone()) {
                    if !(*Level_Complete) {
                        unsafe {
                            raylib::ffi::SetSoundVolume(*Level_Complete_Sound, 0.5);
                            raylib::ffi::PlaySound(*Level_Complete_Sound);
                            *Level_Complete = true;
                        }
                    }
                }
            }
        }
    }

}

fn Draw_Ball (Index: i32, Ball: Ball_Type) {
    unsafe {
        raylib::ffi::DrawCircle (Ball.X, Ball.Y, Ball_Radius, Color::RED.into());
        raylib::ffi::DrawCircle (Ball.X, Ball.Y, Ball_Fill, Color::RAYWHITE.into());

        if Debug {
            let index_string = CString::new(Index.to_string()).unwrap();
            raylib::ffi::DrawText (index_string.as_ptr() as *const i8, Ball.X, Ball.Y, 25, Color::BLACK.into())
        }
    }
}

fn Draw_Game 
    (Balls: &mut Vec<Ball_Type>,
     Clear: bool) {
    unsafe {
        raylib::ffi::BeginDrawing;
        raylib::ffi::ClearBackground (Color::RAYWHITE.into());

        if Balls.len() < 2 {
            panic!()
        }

        let mut Index: i32 = 1;
        let mut Color: raylib::prelude::Color;

        if Clear {
           Color = Color::GREEN; 
        } else {
           Color = Color::BLACK; 
        }
        for Ball_Index in 1 .. Balls.len() - 1
            // For loops do not iterate over each element in the range specified. So an additional
            // range unit is added to the end (+1).
            + 1 
        {
            raylib::ffi::DrawLine (Balls[Ball_Index].X as i32,
                                   Balls[Ball_Index].Y as i32,
                                   Balls[Ball_Index - 1].X as i32,
                                   Balls[Ball_Index - 1].Y as i32,
                                   Color.into());
        }
        raylib::ffi::DrawLine (Balls[0].X as i32,
                               Balls[0].Y as i32,
                               Balls[Balls.len() - 1].X as i32,
                               Balls[Balls.len() - 1].Y as i32,
                               Color.into());
        for Ball in Balls {
            Draw_Ball (Index, Ball.clone());
            Index = Index + 1;
        }

        raylib::ffi::EndDrawing();
    }
}

fn main() {
    unsafe {
        let mut Balls                : Vec<Ball_Type> = Vec::new();
        let mut Movement_State = Movement_State_Type::Standby;
        let mut Movement_Operation   : Movement_Operation_Type = Movement_Operation_Type 
                                        { Ball: 0,
                                          Mouse_X_Start: 0,
                                          Mouse_Y_Start: 0,
                                          Ball_X_Start: 0,
                                          Ball_Y_Start: 0 };
        let mut Clear                : bool = false;
        let mut Level_Complete       : bool = false;
        let Level_Complete_Sound_Name = CString::new("level_complete.wav").unwrap();

        for Not_Used in 1 .. Starting_Balls
            // For loops do not iterate over each element in the range specified. So an additional
            // range unit is added to the end (+1).
            + 1
        {
            Balls.push(Ball_Type { X: Random_X(), Y: Random_Y() });
        }

        New_Level (&mut Balls, &mut Level_Complete);

        let (mut rl, thread) = raylib::init()
            .size(Game_Width, Game_Height)
            .title("Test Raylib - Rust")
            .build();

        raylib::ffi::InitAudioDevice();
        raylib::ffi::SetMasterVolume(0.3);

        if raylib::ffi::IsAudioDeviceReady() {
            println!("Audio device ready!");
        } else {
            println!("Audio device NOT ready!");
        }
        
        let mut Level_Complete_Wave  : raylib::ffi::Wave  = raylib::ffi::LoadWave(Level_Complete_Sound_Name.as_ptr() as *const i8);
        let mut Level_Complete_Sound : raylib::ffi::Sound = raylib::ffi::LoadSoundFromWave(Level_Complete_Wave);

        raylib::ffi::SetTargetFPS (100);

        while (!raylib::ffi::WindowShouldClose()) {
            Update_Game(&mut Balls,
                        &mut Level_Complete,
                        &rl,
                        &mut Movement_State,
                        &mut Movement_Operation,
                        &mut Clear,
                        &mut Level_Complete_Sound);
            Draw_Game(&mut Balls, Clear);
        }
    }
}
