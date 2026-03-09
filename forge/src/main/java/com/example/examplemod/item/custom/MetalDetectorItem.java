package com.example.examplemod.item.custom;

import com.example.examplemod.sound.ModSounds;
import com.example.examplemod.util.ModTags;
import java.util.List;
import javax.annotation.Nullable;
import net.minecraft.client.resources.language.I18n;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.sounds.SoundSource;
import net.minecraft.world.InteractionResult;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.item.Item;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.TooltipFlag;
import net.minecraft.world.item.context.UseOnContext;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;

public class MetalDetectorItem extends Item {
  public MetalDetectorItem(Properties pProperties) {
    super(pProperties);
  }

  @Override
  public InteractionResult useOn(UseOnContext pContext) {
    if (!pContext.getLevel().isClientSide()) {
      BlockPos pos = pContext.getClickedPos();
      Player player = pContext.getPlayer();

      boolean metalDetected = false;

      for (int i = 0; i <= pos.getY() + 999; i++) {
        BlockState state = pContext.getLevel().getBlockState(pos.below(i));
        if (isValuableMetal(state)) {
          outputCoordinates(player, pos.below(i), state.getBlock());
          metalDetected = true;

          pContext
              .getLevel()
              .playSeededSound(
                  null,
                  pos.getX(),
                  pos.getY(),
                  pos.getZ(),
                  ModSounds.METAL_DETECTOR_FOUND_ORE.get(),
                  SoundSource.BLOCKS,
                  0.5f,
                  0.5f,
                  0);
          break;
        }
      }
      if (!metalDetected) {
        player.sendSystemMessage(Component.literal("No valuable metals detected within range."));
      }
    }

    pContext
        .getItemInHand()
        .hurtAndBreak(
            1,
            pContext.getPlayer(),
            (player) -> {
              player.broadcastBreakEvent(pContext.getHand());
            });

    return InteractionResult.SUCCESS;
  }

  @Override
  public void appendHoverText(
      ItemStack pStack,
      @Nullable Level pLevel,
      List<Component> pTooltipComponents,
      TooltipFlag pIsAdvanced) {
    pTooltipComponents.add(Component.translatable("tooltip.metal_detector.description"));
    super.appendHoverText(pStack, pLevel, pTooltipComponents, pIsAdvanced);
  }

  private void outputCoordinates(Player player, BlockPos pos, Block block) {
    player.sendSystemMessage(
        net.minecraft.network.chat.Component.literal(
            "Metal detected: "
                + I18n.get(block.getDescriptionId())
                + " at coordinates X: "
                + pos.getX()
                + " Y: "
                + pos.getY()
                + " Z: "
                + pos.getZ()));
  }

  private boolean isValuableMetal(BlockState state) {
    return state.is(ModTags.Blocks.METAL_DETECTOR_VALUABLES);
  }
}
