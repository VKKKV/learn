package com.example.examplemod.item;

import net.minecraft.world.effect.MobEffectInstance;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.world.food.FoodProperties;

public class ModFoods {
    public static final FoodProperties STRAWBERRY =
            new FoodProperties.Builder()
                    .nutrition(2)
                    .fast()
                    .saturationMod(0.3f)
                    .effect(() -> new MobEffectInstance(MobEffects.MOVEMENT_SPEED, 200), 0.1f)
                    .build();

    public static final FoodProperties CORN =
            new FoodProperties.Builder()
                    .nutrition(1)
                    .fast()
                    .saturationMod(0.1f)
                    .effect(() -> new MobEffectInstance(MobEffects.MOVEMENT_SPEED, 200), 0.1f)
                    .build();
}
